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
	.video-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 8px;
		overflow: hidden;
		transition:
			box-shadow 0.2s ease,
			transform 0.2s ease;
		contain: layout style;
	}

	.video-card:hover {
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	/* Thumbnail Container */
	.video-card__thumbnail-link {
		display: block;
		text-decoration: none;
	}

	.video-card__thumbnail {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 */
		margin: 0;
		overflow: hidden;
		background: #1a1a1a;
	}

	/* Blurhash Layer */
	.video-card__blurhash {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		filter: blur(20px);
		transform: scale(1.1);
		z-index: 1;
	}

	/* Actual Image */
	.video-card__image {
		position: absolute;
		top: 0;
		left: 0;
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

	/* Duration Badge */
	.video-card__duration {
		position: absolute;
		bottom: 8px;
		right: 8px;
		background: rgba(0, 0, 0, 0.85);
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		z-index: 3;
	}

	/* Play Overlay */
	.video-card__play-overlay {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 64px;
		height: 64px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: white;
		opacity: 0;
		transition: opacity 0.2s ease;
		z-index: 4;
	}

	.video-card__play-overlay svg {
		margin-left: 4px;
	}

	.video-card:hover .video-card__play-overlay {
		opacity: 1;
	}

	/* Content Area */
	.video-card__content {
		display: flex;
		flex-direction: column;
		flex: 1;
		padding: 16px;
	}

	/* Tags */
	.video-card__tags {
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
		margin-bottom: 12px;
	}

	.video-card__tag {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 4px;
		color: white;
	}

	/* Title */
	.video-card__title {
		margin: 0 0 8px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.video-card__title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.video-card__title a:hover {
		color: #0984ae;
	}

	/* Meta */
	.video-card__meta {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		font-size: 13px;
		color: #666;
		margin-bottom: 12px;
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
		margin-right: 8px;
		color: #ccc;
	}

	/* Description */
	.video-card__description {
		margin: 0 0 16px;
		font-size: 14px;
		color: #666;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
		flex: 1;
	}

	/* CTA Button */
	.video-card__cta {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 20px;
		background: #f5f5f5;
		color: #333;
		font-size: 13px;
		font-weight: 600;
		text-transform: uppercase;
		text-decoration: none;
		border-radius: 6px;
		transition:
			background 0.2s ease,
			color 0.2s ease;
		margin-top: auto;
		width: fit-content;
	}

	.video-card__cta:hover {
		background: #0984ae;
		color: #fff;
	}

	/* Responsive */
	@media (max-width: 640px) {
		.video-card__content {
			padding: 12px;
		}

		.video-card__title {
			font-size: 14px;
		}

		.video-card__play-overlay {
			width: 48px;
			height: 48px;
		}

		.video-card__play-overlay svg {
			width: 32px;
			height: 32px;
		}
	}

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
</style>
