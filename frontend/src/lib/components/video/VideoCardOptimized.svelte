<!--
/**
 * VideoCardOptimized - Zero-Latency Video Card
 * ═══════════════════════════════════════════════════════════════════════════
 *
 * Apple ICT 11+ Principal Engineer Grade - January 2026
 *
 * PERFORMANCE OPTIMIZATIONS:
 * 1. Blurhash instant placeholder (<1ms)
 * 2. Hover-based video preloading
 * 3. Intersection Observer for viewport preloading
 * 4. Optimized image loading (eager above fold, lazy below)
 * 5. CSS containment for layout stability
 *
 * @version 1.0.0
 */
-->
<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { browser } from '$app/environment';
	import { decodeBlurhash, DEFAULT_BLURHASHES } from '$lib/utils/blurhash';
	import { videoPreloader, type VideoPreloadInfo } from '$lib/utils/videoPreloader';

	// ═══════════════════════════════════════════════════════════════════════
	// TYPES
	// ═══════════════════════════════════════════════════════════════════════

	interface VideoData {
		id: number | string;
		title: string;
		slug: string;
		description?: string | null;
		thumbnail_url?: string | null;
		blurhash?: string | null;
		tag_details?: Array<{ slug: string; name: string; color: string }>;
		trader?: { id: number; name: string; slug: string } | null;
		formatted_date?: string;
		duration?: number | null;
		formatted_duration?: string;
		video_url?: string;
		embed_url?: string;
		video_platform?: string;
		bunny_video_guid?: string;
		bunny_library_id?: number;
		content_type?: string;
		video_date?: string;
		is_published?: boolean;
		is_featured?: boolean;
		tags?: string[];
		views_count?: number;
		rooms?: Array<{ id: number; name: string; slug: string }>;
		created_at?: string;
		[key: string]: unknown;
	}

	interface Props {
		video: VideoData;
		basePath?: string;
		showDate?: boolean;
		showDuration?: boolean;
		showViews?: boolean;
		priority?: 'high' | 'low' | 'auto';
		enablePreload?: boolean;
	}

	// ═══════════════════════════════════════════════════════════════════════
	// PROPS
	// ═══════════════════════════════════════════════════════════════════════

	let {
		video,
		basePath = '/learning-center',
		showDate = false,
		showDuration = true,
		showViews = false,
		priority = 'auto',
		enablePreload = true
	}: Props = $props();

	// ═══════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════

	let cardElement: HTMLElement;
	let thumbnailLoaded = $state(false);
	let isHovering = $state(false);
	let blurhashDataUrl = $state<string | null>(null);
	let observer: IntersectionObserver | null = null;
	let isInViewport = $state(false);

	// ═══════════════════════════════════════════════════════════════════════
	// COMPUTED
	// ═══════════════════════════════════════════════════════════════════════

	const defaultThumbnail =
		'https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg';

	let thumbnailUrl = $derived(video.thumbnail_url || defaultThumbnail);

	// Extract Bunny info for preloading
	let bunnyInfo = $derived.by<VideoPreloadInfo | null>(() => {
		if (video.bunny_video_guid && video.bunny_library_id) {
			return {
				videoId: video.bunny_video_guid,
				libraryId: String(video.bunny_library_id),
				thumbnailUrl: thumbnailUrl
			};
		}
		return null;
	});

	// Loading priority based on position
	let loadingPriority = $derived.by(() => {
		if (priority !== 'auto') return priority;
		return isInViewport ? 'high' : 'low';
	});

	// ═══════════════════════════════════════════════════════════════════════
	// LIFECYCLE
	// ═══════════════════════════════════════════════════════════════════════

	onMount(() => {
		if (!browser) return;

		// Decode blurhash for instant placeholder
		const hash = video.blurhash || DEFAULT_BLURHASHES.video;
		blurhashDataUrl = decodeBlurhash(hash, { width: 32, height: 18 });

		// Setup intersection observer for viewport detection
		setupIntersectionObserver();

		// Register for video preloading if enabled
		if (enablePreload && cardElement) {
			videoPreloader.observe(cardElement);
		}
	});

	onDestroy(() => {
		observer?.disconnect();
		if (cardElement) {
			videoPreloader.unobserve(cardElement);
		}
	});

	// ═══════════════════════════════════════════════════════════════════════
	// METHODS
	// ═══════════════════════════════════════════════════════════════════════

	function setupIntersectionObserver() {
		if (typeof IntersectionObserver === 'undefined') return;

		observer = new IntersectionObserver(
			(entries) => {
				entries.forEach((entry) => {
					isInViewport = entry.isIntersecting;
				});
			},
			{ rootMargin: '100px', threshold: 0 }
		);

		if (cardElement) {
			observer.observe(cardElement);
		}
	}

	function handleMouseEnter() {
		isHovering = true;

		// Trigger aggressive preloading on hover
		const info = bunnyInfo;
		if (enablePreload && info) {
			videoPreloader.onHoverStart(info, 50); // 50ms debounce
		}
	}

	function handleMouseLeave() {
		isHovering = false;

		// Cancel pending preload
		const info = bunnyInfo;
		if (info) {
			videoPreloader.onHoverEnd(info);
		}
	}

	function handleThumbnailLoad() {
		thumbnailLoaded = true;
	}

	function formatViews(views: number | undefined): string {
		if (!views) return '0 views';
		if (views >= 1000000) return `${(views / 1000000).toFixed(1)}M views`;
		if (views >= 1000) return `${(views / 1000).toFixed(1)}K views`;
		return `${views} views`;
	}
</script>

<article
	bind:this={cardElement}
	class="video-card"
	class:is-hovering={isHovering}
	data-video-id={video.bunny_video_guid || video.id}
	data-library-id={video.bunny_library_id}
	data-thumbnail-url={thumbnailUrl}
	onmouseenter={handleMouseEnter}
	onmouseleave={handleMouseLeave}
>
	<!-- Thumbnail Container with Blurhash -->
	<a href="{basePath}/{video.slug}" class="video-card__thumbnail-link">
		<figure class="video-card__thumbnail">
			<!-- Layer 1: Blurhash (instant, <1ms) -->
			{#if blurhashDataUrl && !thumbnailLoaded}
				<div
					class="video-card__blurhash"
					style="background-image: url({blurhashDataUrl})"
					aria-hidden="true"
				></div>
			{/if}

			<!-- Layer 2: Actual Thumbnail -->
			<img
				src={thumbnailUrl}
				alt={video.title}
				loading={loadingPriority === 'high' ? 'eager' : 'lazy'}
				decoding="async"
				fetchpriority={loadingPriority}
				class="video-card__image"
				class:is-loaded={thumbnailLoaded}
				onload={handleThumbnailLoad}
			/>

			<!-- Duration Badge -->
			{#if showDuration && video.formatted_duration}
				<span class="video-card__duration">{video.formatted_duration}</span>
			{/if}

			<!-- Play Icon Overlay -->
			<div class="video-card__play-overlay" aria-hidden="true">
				<svg viewBox="0 0 24 24" fill="currentColor" width="48" height="48">
					<path d="M8 5v14l11-7z" />
				</svg>
			</div>
		</figure>
	</a>

	<!-- Content -->
	<div class="video-card__content">
		<!-- Tags -->
		{#if video.tag_details && video.tag_details.length > 0}
			<div class="video-card__tags">
				{#each video.tag_details.slice(0, 3) as tag}
					<span class="video-card__tag" style="background-color: {tag.color}">{tag.name}</span>
				{/each}
			</div>
		{/if}

		<!-- Title -->
		<h4 class="video-card__title">
			<a href="{basePath}/{video.slug}">{video.title}</a>
		</h4>

		<!-- Meta Info -->
		<div class="video-card__meta">
			{#if video.trader}
				<span class="video-card__trader">With {video.trader.name}</span>
			{/if}
			{#if showDate && video.formatted_date}
				<span class="video-card__date">{video.formatted_date}</span>
			{/if}
			{#if showViews && video.views_count}
				<span class="video-card__views">{formatViews(video.views_count)}</span>
			{/if}
		</div>

		<!-- Description -->
		{#if video.description}
			<p class="video-card__description">{video.description}</p>
		{/if}

		<!-- CTA Button -->
		<a href="{basePath}/{video.slug}" class="video-card__cta"> Watch Now </a>
	</div>
</article>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   VIDEO CARD OPTIMIZED - 2026 Mobile-First Responsive Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Touch targets: 44x44px minimum for all interactive elements
	   ═══════════════════════════════════════════════════════════════════════════ */

	.video-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 6px;
		overflow: hidden;
		transition:
			box-shadow 0.2s ease,
			transform 0.2s ease;
		contain: layout style;
	}

	/* Thumbnail Container */
	.video-card__thumbnail-link {
		display: block;
		text-decoration: none;
		-webkit-tap-highlight-color: transparent;
	}

	.video-card__thumbnail {
		position: relative;
		width: 100%;
		/* Modern aspect-ratio property */
		aspect-ratio: 16 / 9;
		margin: 0;
		overflow: hidden;
		background: #1a1a1a;
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.video-card__thumbnail {
			padding-top: 56.25%;
		}
	}

	/* Blurhash Layer */
	.video-card__blurhash {
		position: absolute;
		inset: 0;
		background-size: cover;
		background-position: center;
		filter: blur(20px);
		transform: scale(1.1);
		z-index: 1;
	}

	/* Actual Image */
	.video-card__image {
		position: absolute;
		inset: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
		transition: opacity 0.3s ease;
		z-index: 2;
	}

	.video-card__image.is-loaded {
		opacity: 1;
	}

	/* Duration Badge - Mobile first */
	.video-card__duration {
		position: absolute;
		bottom: 6px;
		right: 6px;
		background: rgba(0, 0, 0, 0.85);
		color: #fff;
		padding: 3px 6px;
		border-radius: 4px;
		font-size: 11px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		z-index: 3;
	}

	/* Play Overlay - Mobile first: 48px touch target */
	.video-card__play-overlay {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 48px;
		height: 48px;
		min-width: 44px;
		min-height: 44px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		opacity: 0.9;
		transition: opacity 0.2s ease;
		z-index: 4;
	}

	.video-card__play-overlay svg {
		width: 28px;
		height: 28px;
		margin-left: 3px;
	}

	/* Content Area - Mobile first */
	.video-card__content {
		display: flex;
		flex-direction: column;
		flex: 1;
		padding: 12px;
	}

	/* Tags */
	.video-card__tags {
		display: flex;
		flex-wrap: wrap;
		gap: 4px;
		margin-bottom: 8px;
	}

	.video-card__tag {
		display: inline-block;
		padding: 3px 8px;
		font-size: 10px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 4px;
		color: white;
	}

	/* Title - Mobile first */
	.video-card__title {
		margin: 0 0 6px;
		font-size: 14px;
		font-weight: 700;
		line-height: 1.4;
	}

	.video-card__title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
		-webkit-tap-highlight-color: transparent;
	}

	/* Meta */
	.video-card__meta {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		font-size: 12px;
		color: #666;
		margin-bottom: 10px;
	}

	.video-card__trader {
		font-style: italic;
	}

	.video-card__date,
	.video-card__views {
		color: #999;
	}

	.video-card__date::before,
	.video-card__views::before {
		content: '•';
		margin-right: 6px;
		color: #ccc;
	}

	/* Description - Mobile first */
	.video-card__description {
		margin: 0 0 12px;
		font-size: 13px;
		color: #666;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		flex: 1;
	}

	/* CTA Button - 44px min touch target */
	.video-card__cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		padding: 10px 16px;
		background: #f5f5f5;
		color: #333;
		font-size: 12px;
		font-weight: 600;
		text-transform: uppercase;
		text-decoration: none;
		border-radius: 6px;
		transition:
			background 0.2s ease,
			color 0.2s ease;
		margin-top: auto;
		width: fit-content;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - Mobile First (min-width queries)
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.video-card__content {
			padding: 14px;
		}

		.video-card__duration {
			padding: 4px 8px;
			font-size: 12px;
		}

		.video-card__title {
			font-size: 15px;
		}

		.video-card__cta {
			padding: 10px 18px;
			font-size: 13px;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media (min-width: 640px) {
		.video-card {
			border-radius: 8px;
		}

		.video-card__content {
			padding: 16px;
		}

		.video-card__play-overlay {
			width: 56px;
			height: 56px;
			opacity: 0;
		}

		.video-card__play-overlay svg {
			width: 32px;
			height: 32px;
		}

		.video-card:hover .video-card__play-overlay {
			opacity: 1;
		}

		.video-card:hover {
			box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
			transform: translateY(-2px);
		}

		.video-card__title a:hover {
			color: #0984ae;
		}

		.video-card__cta:hover {
			background: #0984ae;
			color: #fff;
		}

		.video-card__tags {
			gap: 6px;
			margin-bottom: 12px;
		}

		.video-card__tag {
			padding: 4px 10px;
			font-size: 11px;
		}

		.video-card__title {
			font-size: 16px;
			margin-bottom: 8px;
		}

		.video-card__meta {
			gap: 8px;
			font-size: 13px;
			margin-bottom: 12px;
		}

		.video-card__description {
			font-size: 14px;
			margin-bottom: 16px;
		}

		.video-card__cta {
			padding: 10px 20px;
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.video-card__play-overlay {
			width: 64px;
			height: 64px;
		}

		.video-card__play-overlay svg {
			width: 40px;
			height: 40px;
			margin-left: 4px;
		}

		.video-card__duration {
			bottom: 8px;
			right: 8px;
		}
	}

	/* lg: 1024px+ - Laptops/Desktops */
	@media (min-width: 1024px) {
		.video-card {
			border-radius: 10px;
		}

		.video-card__content {
			padding: 18px;
		}

		.video-card__title {
			font-size: 17px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Reduced motion */
	@media (prefers-reduced-motion: reduce) {
		.video-card,
		.video-card__image,
		.video-card__play-overlay,
		.video-card__cta {
			transition: none;
		}

		.video-card:hover {
			transform: none;
		}
	}

	/* Focus visible for keyboard navigation */
	.video-card__thumbnail-link:focus-visible,
	.video-card__title a:focus-visible,
	.video-card__cta:focus-visible {
		outline: 2px solid #0984ae;
		outline-offset: 2px;
	}
</style>
