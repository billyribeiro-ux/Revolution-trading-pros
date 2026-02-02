<!--
/**
 * Video Block Component
 * ============================================================================
 * Production-ready video player with YouTube, Vimeo, and native video support.
 * Features auto-detection of video platforms, responsive 16:9 embed container,
 * editable captions, and comprehensive dark mode support.
 */
-->

<script lang="ts">
	import { IconVideo } from '$lib/icons';
	import { sanitizeURL } from '$lib/utils/sanitization';
	import type { Block, BlockContent } from '../types';
	import type { BlockId } from '$lib/stores/blockState.svelte';

	// ==========================================================================
	// Type Definitions
	// ==========================================================================

	type VideoType = 'youtube' | 'vimeo' | 'native';

	// ==========================================================================
	// Props
	// ==========================================================================

	interface Props {
		block: Block;
		blockId: BlockId;
		isSelected: boolean;
		isEditing: boolean;
		onUpdate: (updates: Partial<Block>) => void;
		onError?: (error: Error) => void;
	}

	let props: Props = $props();

	// ==========================================================================
	// Local State
	// ==========================================================================

	let urlInputValue = $state(props.block.content.mediaUrl || '');
	let isLoading = $state(false);
	let hasError = $state(false);
	let errorMessage = $state('');

	// ==========================================================================
	// Derived State with $derived.by()
	// ==========================================================================

	const mediaUrl = $derived(props.block.content.mediaUrl || '');
	const mediaCaption = $derived(props.block.content.mediaCaption || '');

	/**
	 * Auto-detect video type from URL
	 * - YouTube (youtube.com, youtu.be) -> 'youtube'
	 * - Vimeo (vimeo.com) -> 'vimeo'
	 * - Other -> 'native'
	 */
	const videoType = $derived.by((): VideoType | null => {
		if (!mediaUrl) return null;

		const url = mediaUrl.toLowerCase();

		// YouTube detection
		if (url.includes('youtube.com') || url.includes('youtu.be')) {
			return 'youtube';
		}

		// Vimeo detection
		if (url.includes('vimeo.com')) {
			return 'vimeo';
		}

		// Default to native for direct video URLs
		return 'native';
	});

	/**
	 * Extract video ID for embeds
	 * - YouTube: /watch?v=ID or youtu.be/ID
	 * - Vimeo: vimeo.com/ID
	 */
	const videoId = $derived.by((): string | null => {
		if (!mediaUrl || !videoType) return null;

		if (videoType === 'youtube') {
			// Match youtube.com/watch?v=ID, youtube.com/embed/ID, youtube.com/v/ID, youtu.be/ID
			const match = mediaUrl.match(
				/(?:youtube\.com\/(?:watch\?v=|embed\/|v\/)|youtu\.be\/)([a-zA-Z0-9_-]{11})/
			);
			return match?.[1] || null;
		}

		if (videoType === 'vimeo') {
			// Match vimeo.com/ID or vimeo.com/video/ID
			const match = mediaUrl.match(/vimeo\.com\/(?:video\/)?(\d+)/);
			return match?.[1] || null;
		}

		return null;
	});

	/**
	 * Generate embed URL based on video type
	 * - YouTube: https://www.youtube-nocookie.com/embed/{id}?rel=0&modestbranding=1
	 * - Vimeo: https://player.vimeo.com/video/{id}?title=0&byline=0&portrait=0
	 * - Native: Use the original URL
	 */
	const embedUrl = $derived.by((): string | null => {
		if (!mediaUrl) return null;

		if (videoType === 'youtube' && videoId) {
			return `https://www.youtube-nocookie.com/embed/${videoId}?rel=0&modestbranding=1`;
		}

		if (videoType === 'vimeo' && videoId) {
			return `https://player.vimeo.com/video/${videoId}?title=0&byline=0&portrait=0`;
		}

		if (videoType === 'native') {
			return sanitizeURL(mediaUrl);
		}

		return null;
	});

	const hasVideo = $derived(!!embedUrl);

	// ==========================================================================
	// Content Update Handlers
	// ==========================================================================

	function updateContent(updates: Partial<BlockContent>): void {
		props.onUpdate({
			content: { ...props.block.content, ...updates }
		});
	}

	function handleUrlInput(e: Event): void {
		const target = e.target as HTMLInputElement;
		urlInputValue = target.value;
	}

	function handleUrlSubmit(): void {
		const trimmed = urlInputValue.trim();
		if (!trimmed) return;

		const sanitized = sanitizeURL(trimmed);
		if (!sanitized) {
			hasError = true;
			errorMessage = 'Invalid URL format. Please enter a valid video URL.';
			props.onError?.(new Error(errorMessage));
			return;
		}

		// Validate that we can parse it as a video
		const url = sanitized.toLowerCase();
		const isYouTube = url.includes('youtube.com') || url.includes('youtu.be');
		const isVimeo = url.includes('vimeo.com');
		const isNative = /\.(mp4|webm|ogg|mov)(\?.*)?$/i.test(url) || url.includes('video/');

		if (!isYouTube && !isVimeo && !isNative) {
			hasError = true;
			errorMessage = 'Unable to recognize video URL. Please use YouTube, Vimeo, or a direct video link.';
			props.onError?.(new Error(errorMessage));
			return;
		}

		hasError = false;
		errorMessage = '';
		isLoading = true;
		updateContent({ mediaUrl: sanitized });
	}

	function handleUrlKeydown(e: KeyboardEvent): void {
		if (e.key === 'Enter') {
			e.preventDefault();
			handleUrlSubmit();
		}
	}

	function handleCaptionInput(e: Event): void {
		const target = e.target as HTMLElement;
		updateContent({ mediaCaption: target.textContent || '' });
	}

	function handlePaste(e: ClipboardEvent): void {
		e.preventDefault();
		const text = e.clipboardData?.getData('text/plain') || '';
		document.execCommand('insertText', false, text);
	}

	function handleIframeLoad(): void {
		isLoading = false;
		hasError = false;
	}

	function handleIframeError(): void {
		isLoading = false;
		hasError = true;
		errorMessage = 'Failed to load video. Please check the URL and try again.';
		props.onError?.(new Error(errorMessage));
	}

	function handleNativeVideoLoad(): void {
		isLoading = false;
		hasError = false;
	}

	function handleNativeVideoError(): void {
		isLoading = false;
		hasError = true;
		errorMessage = 'Failed to load video file. The format may not be supported.';
		props.onError?.(new Error(errorMessage));
	}

	function clearVideo(): void {
		urlInputValue = '';
		hasError = false;
		errorMessage = '';
		isLoading = false;
		updateContent({ mediaUrl: '', mediaCaption: '' });
	}

	// ==========================================================================
	// Reset loading state when URL changes
	// ==========================================================================

	$effect(() => {
		if (mediaUrl) {
			isLoading = true;
			hasError = false;
		}
	});

	// Sync input value with block content
	$effect(() => {
		urlInputValue = props.block.content.mediaUrl || '';
	});
</script>

<div
	class="video-block"
	class:video-block--selected={props.isSelected}
	class:video-block--editing={props.isEditing}
	role="region"
	aria-label="Video player"
	data-block-id={props.blockId}
>
	{#if hasVideo && embedUrl}
		<!-- Video Container with 16:9 Aspect Ratio -->
		<div class="video-container">
			<!-- Loading State -->
			{#if isLoading}
				<div class="video-loading" aria-live="polite">
					<div class="video-loading-spinner" aria-hidden="true"></div>
					<span class="visually-hidden">Loading video...</span>
				</div>
			{/if}

			<!-- Error State -->
			{#if hasError}
				<div class="video-error" role="alert">
					<IconVideo size={32} aria-hidden="true" />
					<span>{errorMessage}</span>
				</div>
			{/if}

			<!-- Native Video Player -->
			{#if videoType === 'native'}
				<video
					src={embedUrl}
					class="video-native"
					class:video-hidden={isLoading || hasError}
					controls
					preload="metadata"
					onloadeddata={handleNativeVideoLoad}
					onerror={handleNativeVideoError}
					aria-label={mediaCaption || 'Video player'}
				>
					<track kind="captions" label="Captions" />
					Your browser does not support the video tag.
				</video>
			{:else}
				<!-- Iframe Embed (YouTube/Vimeo) -->
				<iframe
					src={embedUrl}
					title={mediaCaption || `${videoType === 'youtube' ? 'YouTube' : 'Vimeo'} video player`}
					class="video-iframe"
					class:video-hidden={isLoading || hasError}
					frameborder="0"
					allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
					allowfullscreen
					loading="lazy"
					onload={handleIframeLoad}
					onerror={handleIframeError}
				></iframe>
			{/if}

			<!-- Edit Mode: Clear Button -->
			{#if props.isEditing && !isLoading}
				<button
					type="button"
					class="video-clear-btn"
					onclick={clearVideo}
					aria-label="Remove video"
				>
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<line x1="18" y1="6" x2="6" y2="18"></line>
						<line x1="6" y1="6" x2="18" y2="18"></line>
					</svg>
				</button>
			{/if}
		</div>

		<!-- Caption -->
		{#if mediaCaption || isEditing}
			<figcaption
				contenteditable={props.isEditing}
				class="video-caption"
				class:video-caption--placeholder={!mediaCaption && props.isEditing}
				oninput={handleCaptionInput}
				onpaste={handlePaste}
				data-placeholder="Add a caption..."
				role={props.isEditing ? 'textbox' : undefined}
				aria-label={props.isEditing ? 'Video caption' : undefined}
				aria-multiline="false"
			>
				{mediaCaption}
			</figcaption>
		{/if}
	{:else if props.isEditing}
		<!-- Placeholder State (Edit Mode) -->
		<div class="video-placeholder">
			<div class="video-placeholder-icon">
				<IconVideo size={48} aria-hidden="true" />
			</div>
			<span class="video-placeholder-title">Add a Video</span>
			<span class="video-placeholder-subtitle">
				Paste a YouTube, Vimeo, or direct video URL
			</span>

			<div class="video-url-input-wrapper">
				<svg class="video-url-icon" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
					<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"></path>
					<path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"></path>
				</svg>
				<input
					type="url"
					placeholder="https://www.youtube.com/watch?v=..."
					value={urlInputValue}
					oninput={handleUrlInput}
					onkeydown={handleUrlKeydown}
					aria-label="Video URL"
					autocomplete="off"
				/>
				<button
					type="button"
					class="video-url-submit"
					onclick={handleUrlSubmit}
					disabled={!urlInputValue.trim()}
					aria-label="Add video"
				>
					<svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<polygon points="5 3 19 12 5 21 5 3"></polygon>
					</svg>
				</button>
			</div>

			{#if hasError}
				<div class="video-input-error" role="alert">
					<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
						<circle cx="12" cy="12" r="10"></circle>
						<line x1="12" y1="8" x2="12" y2="12"></line>
						<line x1="12" y1="16" x2="12.01" y2="16"></line>
					</svg>
					<span>{errorMessage}</span>
				</div>
			{/if}

			<div class="video-supported-platforms">
				<span class="video-platform">YouTube</span>
				<span class="video-platform-divider">|</span>
				<span class="video-platform">Vimeo</span>
				<span class="video-platform-divider">|</span>
				<span class="video-platform">MP4/WebM</span>
			</div>
		</div>
	{:else}
		<!-- Empty State (View Mode) -->
		<div class="video-empty" role="status">
			<IconVideo size={32} aria-hidden="true" />
			<span>No video available</span>
		</div>
	{/if}
</div>

<style>
	/* ==========================================================================
	   Base Container
	   ========================================================================== */

	.video-block {
		width: 100%;
		border-radius: 12px;
		overflow: hidden;
	}

	.video-block--selected {
		outline: 2px solid #3b82f6;
		outline-offset: 2px;
	}

	/* ==========================================================================
	   Video Container (16:9 Aspect Ratio via padding-bottom: 56.25%)
	   ========================================================================== */

	.video-container {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background: #0f172a;
		border-radius: 12px;
		overflow: hidden;
	}

	.video-iframe,
	.video-native {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		border: none;
		transition: opacity 0.3s ease;
	}

	.video-hidden {
		opacity: 0;
	}

	/* ==========================================================================
	   Loading State
	   ========================================================================== */

	.video-loading {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
		z-index: 10;
	}

	.video-loading-spinner {
		width: 32px;
		height: 32px;
		border: 3px solid #334155;
		border-top-color: #3b82f6;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		from {
			transform: rotate(0deg);
		}
		to {
			transform: rotate(360deg);
		}
	}

	/* ==========================================================================
	   Error State
	   ========================================================================== */

	.video-error {
		position: absolute;
		inset: 0;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		background: linear-gradient(135deg, #450a0a 0%, #1e1e1e 100%);
		color: #fca5a5;
		font-size: 0.875rem;
		text-align: center;
		padding: 1rem;
		z-index: 10;
	}

	/* ==========================================================================
	   Clear Button (Edit Mode)
	   ========================================================================== */

	.video-clear-btn {
		position: absolute;
		top: 0.75rem;
		right: 0.75rem;
		width: 32px;
		height: 32px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.7);
		border: none;
		border-radius: 50%;
		color: white;
		cursor: pointer;
		opacity: 0;
		transition: all 0.2s ease;
		z-index: 20;
	}

	.video-block--editing .video-container:hover .video-clear-btn {
		opacity: 1;
	}

	.video-clear-btn:hover {
		background: rgba(239, 68, 68, 0.9);
		transform: scale(1.1);
	}

	.video-clear-btn:focus-visible {
		opacity: 1;
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	/* ==========================================================================
	   Caption (Contenteditable)
	   ========================================================================== */

	.video-caption {
		margin-top: 0.75rem;
		font-size: 0.875rem;
		color: #6b7280;
		line-height: 1.5;
		text-align: center;
		outline: none;
		min-height: 1.5em;
	}

	.video-caption:focus {
		color: #374151;
	}

	.video-caption--placeholder:empty::before {
		content: attr(data-placeholder);
		color: #9ca3af;
	}

	/* ==========================================================================
	   Placeholder State (Edit Mode)
	   ========================================================================== */

	.video-placeholder {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		gap: 0.75rem;
		padding: 3rem 2rem;
		background: #f9fafb;
		border: 2px dashed #d1d5db;
		border-radius: 12px;
		color: #6b7280;
		text-align: center;
	}

	.video-placeholder-icon {
		color: #9ca3af;
	}

	.video-placeholder-title {
		font-size: 1.125rem;
		font-weight: 600;
		color: #374151;
	}

	.video-placeholder-subtitle {
		font-size: 0.875rem;
		color: #9ca3af;
	}

	.video-url-input-wrapper {
		display: flex;
		align-items: center;
		gap: 0.75rem;
		width: 100%;
		max-width: 500px;
		padding: 0.75rem 1rem;
		margin-top: 0.5rem;
		background: white;
		border: 1px solid #d1d5db;
		border-radius: 12px;
		transition: all 0.15s ease;
	}

	.video-url-input-wrapper:focus-within {
		border-color: #3b82f6;
		box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
	}

	.video-url-icon {
		color: #94a3b8;
		flex-shrink: 0;
	}

	.video-url-input-wrapper input {
		flex: 1;
		border: none;
		font-size: 0.9375rem;
		outline: none;
		background: transparent;
		color: #1f2937;
	}

	.video-url-input-wrapper input::placeholder {
		color: #9ca3af;
	}

	.video-url-submit {
		width: 36px;
		height: 36px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #3b82f6;
		border: none;
		border-radius: 8px;
		color: white;
		cursor: pointer;
		flex-shrink: 0;
		transition: all 0.15s ease;
	}

	.video-url-submit:hover:not(:disabled) {
		background: #2563eb;
		transform: scale(1.05);
	}

	.video-url-submit:disabled {
		background: #e5e7eb;
		color: #9ca3af;
		cursor: not-allowed;
	}

	.video-url-submit:focus-visible {
		outline: 2px solid #60a5fa;
		outline-offset: 2px;
	}

	.video-input-error {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 0.5rem;
		padding: 0.5rem 1rem;
		background: #fef2f2;
		border-radius: 8px;
		color: #dc2626;
		font-size: 0.8125rem;
	}

	.video-supported-platforms {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		margin-top: 1rem;
		padding-top: 1rem;
		border-top: 1px solid #e5e7eb;
		font-size: 0.75rem;
		color: #9ca3af;
	}

	.video-platform-divider {
		color: #d1d5db;
	}

	/* ==========================================================================
	   Empty State (View Mode)
	   ========================================================================== */

	.video-empty {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.75rem;
		padding: 3rem;
		background: #f9fafb;
		border: 1px solid #e5e7eb;
		border-radius: 12px;
		color: #6b7280;
	}

	/* ==========================================================================
	   Accessibility
	   ========================================================================== */

	.visually-hidden {
		position: absolute;
		width: 1px;
		height: 1px;
		padding: 0;
		margin: -1px;
		overflow: hidden;
		clip: rect(0, 0, 0, 0);
		white-space: nowrap;
		border: 0;
	}

	/* ==========================================================================
	   Responsive Design
	   ========================================================================== */

	@media (max-width: 640px) {
		.video-placeholder {
			padding: 2rem 1rem;
		}

		.video-url-input-wrapper {
			flex-wrap: wrap;
		}

		.video-supported-platforms {
			gap: 0.375rem;
		}
	}

	/* ==========================================================================
	   Dark Mode Support
	   ========================================================================== */

	:global(.dark) .video-block {
		background: transparent;
	}

	:global(.dark) .video-container {
		background: #0f172a;
	}

	:global(.dark) .video-caption {
		color: #94a3b8;
	}

	:global(.dark) .video-caption:focus {
		color: #e2e8f0;
	}

	:global(.dark) .video-caption--placeholder:empty::before {
		color: #64748b;
	}

	:global(.dark) .video-placeholder {
		background: #1e293b;
		border-color: #334155;
		color: #94a3b8;
	}

	:global(.dark) .video-placeholder-title {
		color: #f1f5f9;
	}

	:global(.dark) .video-placeholder-subtitle {
		color: #64748b;
	}

	:global(.dark) .video-url-input-wrapper {
		background: #0f172a;
		border-color: #475569;
	}

	:global(.dark) .video-url-input-wrapper input {
		color: #e2e8f0;
	}

	:global(.dark) .video-url-input-wrapper input::placeholder {
		color: #64748b;
	}

	:global(.dark) .video-url-icon {
		color: #64748b;
	}

	:global(.dark) .video-url-submit:disabled {
		background: #334155;
		color: #64748b;
	}

	:global(.dark) .video-input-error {
		background: #450a0a;
		color: #fca5a5;
	}

	:global(.dark) .video-supported-platforms {
		border-color: #334155;
	}

	:global(.dark) .video-platform-divider {
		color: #475569;
	}

	:global(.dark) .video-empty {
		background: #1e293b;
		border-color: #334155;
		color: #94a3b8;
	}
</style>
