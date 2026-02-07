<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoGrid Component - Latest Video Updates Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Grid display for recent videos with featured video + modal playback
	 * @version 5.0.0 - Modal Video Playback + View More
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';
	import VideoCard from './VideoCard.svelte';
	import VideoModal from './VideoModal.svelte';

	interface Props {
		videos: Video[];
		isLoading?: boolean;
		title?: string;
		subtitle?: string;
		roomSlug?: string;
	}

	const {
		videos,
		isLoading = false,
		title: _title = '',
		subtitle: _subtitle = '',
		roomSlug = 'explosive-swings'
	}: Props = $props();
	void _title;
	void _subtitle;

	const featuredVideo = $derived(videos.find((v) => v.isFeatured) || videos[0]);
	const gridVideos = $derived(videos.filter((v) => v !== featuredVideo).slice(0, 3));
	const hasMoreVideos = $derived(videos.length > 4);

	// Video Modal State
	let selectedVideo = $state<Video | null>(null);
	let isModalOpen = $state(false);

	function openVideoModal(video: Video) {
		selectedVideo = video;
		isModalOpen = true;
	}

	function closeVideoModal() {
		isModalOpen = false;
		selectedVideo = null;
	}
</script>

<div class="video-grid-container">
	{#if isLoading}
		<!-- Loading State -->
		<div class="loading-container">
			<div class="skeleton-featured">
				<div class="skeleton-play"></div>
			</div>
			<div class="skeleton-grid">
				{#each [1, 2, 3] as _, i}
					<div class="skeleton-card" style="animation-delay: {i * 0.1}s">
						<div class="skeleton-thumb"></div>
						<div class="skeleton-info">
							<div class="skeleton-title"></div>
							<div class="skeleton-date"></div>
						</div>
					</div>
				{/each}
			</div>
		</div>
	{:else if videos.length === 0}
		<!-- Empty State -->
		<div class="empty-state">
			<div class="empty-icon">
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="1.5"
					aria-hidden="true"
				>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
					/>
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M15.91 11.672a.375.375 0 010 .656l-5.603 3.113a.375.375 0 01-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112z"
					/>
				</svg>
			</div>
			<h3 class="empty-title">No video updates yet</h3>
			<p class="empty-text">Check back soon for trade breakdowns and analysis videos.</p>
		</div>
	{:else}
		<!-- Videos Layout -->
		<div class="videos-layout">
			<!-- Featured Video -->
			{#if featuredVideo}
				<div class="featured-container">
					<div class="featured-badge">
						<svg viewBox="0 0 20 20" fill="currentColor" class="badge-icon" aria-hidden="true">
							<path
								fill-rule="evenodd"
								d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z"
								clip-rule="evenodd"
							/>
						</svg>
						<span>Featured</span>
					</div>
					<button class="featured-card-wrapper" onclick={() => openVideoModal(featuredVideo)}>
						<VideoCard video={featuredVideo} variant="featured" />
					</button>
				</div>
			{/if}

			<!-- Grid Videos -->
			{#if gridVideos.length > 0}
				<div class="grid-container">
					{#each gridVideos as video (video.id)}
						<button class="video-card-btn" onclick={() => openVideoModal(video)}>
							<VideoCard {video} variant="default" />
						</button>
					{/each}
				</div>
			{/if}

			<!-- View More Button -->
			{#if hasMoreVideos || videos.length > 0}
				<div class="view-more-container">
					<a href="/dashboard/{roomSlug}/video-library" class="view-more-btn">
						<span>View All Videos</span>
						<svg viewBox="0 0 20 20" fill="currentColor" class="view-more-icon">
							<path
								fill-rule="evenodd"
								d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z"
								clip-rule="evenodd"
							/>
						</svg>
					</a>
				</div>
			{/if}
		</div>
	{/if}
</div>

<!-- Video Modal -->
<VideoModal video={selectedVideo} isOpen={isModalOpen} onClose={closeVideoModal} />

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   CONTAINER
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-grid-container {
		width: 100%;
		padding-bottom: 48px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEOS LAYOUT
	   ═══════════════════════════════════════════════════════════════════════ */
	.videos-layout {
		display: flex;
		flex-direction: column;
		gap: 40px;
	}

	/* Featured Video Container */
	.featured-container {
		position: relative;
		max-width: 880px;
		margin: 0 auto;
		width: 100%;
		padding-top: 12px;
	}

	.featured-badge {
		position: absolute;
		top: 0;
		left: 20px;
		z-index: 10;
		display: inline-flex;
		align-items: center;
		gap: 6px;
		padding: 8px 14px;
		background: linear-gradient(135deg, var(--color-brand-secondary) 0%, #e07d0a 100%);
		color: var(--color-bg-card);
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.06em;
		border-radius: 8px;
		box-shadow:
			0 4px 12px rgba(246, 149, 50, 0.4),
			0 1px 3px rgba(0, 0, 0, 0.1);
	}

	.badge-icon {
		width: 14px;
		height: 14px;
	}

	.featured-card-wrapper {
		border-radius: 16px;
		overflow: hidden;
	}

	/* Video Grid - 3 columns below featured */
	.grid-container {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
		padding: 0 24px;
		margin-bottom: 8px;
	}

	/* Video Card Button Wrapper */
	.video-card-btn,
	.featured-card-wrapper {
		all: unset;
		display: block;
		width: 100%;
		cursor: pointer;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIEW MORE BUTTON
	   ═══════════════════════════════════════════════════════════════════════ */
	.view-more-container {
		display: flex;
		justify-content: center;
		padding-top: 24px;
	}

	.view-more-btn {
		display: inline-flex;
		align-items: center;
		gap: 10px;
		padding: 14px 28px;
		font-size: 14px;
		font-weight: 600;
		color: var(--color-bg-card);
		background: linear-gradient(
			135deg,
			var(--color-brand-primary) 0%,
			var(--color-brand-primary-light) 100%
		);
		border: none;
		border-radius: 12px;
		text-decoration: none;
		cursor: pointer;
		transition: all 0.25s ease-out;
		box-shadow: 0 4px 14px rgba(20, 62, 89, 0.25);
	}

	.view-more-btn:hover {
		transform: translateY(-2px);
		box-shadow: 0 8px 24px rgba(20, 62, 89, 0.35);
		background: linear-gradient(135deg, #1a4d6e 0%, #246089 100%);
	}

	.view-more-btn:active {
		transform: translateY(0);
	}

	.view-more-icon {
		width: 18px;
		height: 18px;
		transition: transform 0.2s ease-out;
	}

	.view-more-btn:hover .view-more-icon {
		transform: translateX(4px);
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════ */
	.loading-container {
		display: flex;
		flex-direction: column;
		gap: 40px;
	}

	.skeleton-featured {
		position: relative;
		width: 100%;
		max-width: 880px;
		margin: 0 auto;
		aspect-ratio: 16 / 9;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
		border-radius: 16px;
		overflow: hidden;
	}

	.skeleton-play {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 72px;
		height: 72px;
		background: rgba(255, 255, 255, 0.25);
		border-radius: 50%;
	}

	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	.skeleton-card {
		border-radius: 14px;
		overflow: hidden;
		background: var(--color-bg-card);
		border: 1px solid var(--color-border-default);
	}

	.skeleton-thumb {
		aspect-ratio: 16 / 9;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
	}

	.skeleton-info {
		padding: 16px;
	}

	.skeleton-title {
		height: 18px;
		width: 85%;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
		border-radius: 6px;
		margin-bottom: 12px;
	}

	.skeleton-date {
		height: 14px;
		width: 45%;
		background: linear-gradient(
			90deg,
			var(--color-bg-subtle) 25%,
			var(--color-border-default) 50%,
			var(--color-bg-subtle) 75%
		);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
		border-radius: 4px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   EMPTY STATE
	   ═══════════════════════════════════════════════════════════════════════ */
	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding: 72px 32px;
		text-align: center;
		background: linear-gradient(180deg, var(--color-bg-subtle) 0%, var(--color-bg-subtle) 100%);
		border: 1px solid var(--color-border-default);
		border-radius: 20px;
	}

	.empty-icon {
		width: 72px;
		height: 72px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--color-border-default);
		border-radius: 18px;
		margin-bottom: 24px;
	}

	.empty-icon svg {
		width: 36px;
		height: 36px;
		color: var(--color-text-muted);
	}

	.empty-title {
		font-size: 18px;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin: 0 0 8px 0;
		letter-spacing: -0.01em;
	}

	.empty-text {
		font-size: 15px;
		color: var(--color-text-muted);
		margin: 0;
		max-width: 320px;
		line-height: 1.5;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS - 2026 Mobile-First Design
	   ═══════════════════════════════════════════════════════════════════════════
	   Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	   Grid: 1 col (mobile) → 2 cols (sm) → 3 cols (lg) → 4 cols (xl)
	   Touch targets: 44x44px minimum for interactive elements
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile first: 1 column grid */
	.grid-container {
		grid-template-columns: 1fr;
		gap: 14px;
		padding: 0 12px;
	}

	.skeleton-grid {
		grid-template-columns: 1fr;
		gap: 14px;
	}

	.videos-layout {
		gap: 24px;
	}

	.featured-container {
		padding-top: 8px;
	}

	.featured-badge {
		left: 12px;
		font-size: 10px;
		padding: 6px 10px;
		gap: 4px;
		border-radius: 6px;
	}

	.badge-icon {
		width: 12px;
		height: 12px;
	}

	.empty-state {
		padding: 48px 20px;
		border-radius: 14px;
	}

	.empty-icon {
		width: 60px;
		height: 60px;
		border-radius: 14px;
		margin-bottom: 18px;
	}

	.empty-icon svg {
		width: 30px;
		height: 30px;
	}

	.empty-title {
		font-size: 16px;
	}

	.empty-text {
		font-size: 13px;
	}

	/* View More Button - 44px touch target */
	.view-more-btn {
		min-height: 44px;
		padding: 12px 20px;
		font-size: 13px;
		border-radius: 10px;
		touch-action: manipulation;
		-webkit-tap-highlight-color: transparent;
	}

	.view-more-icon {
		width: 16px;
		height: 16px;
	}

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.grid-container {
			padding: 0 14px;
		}

		.featured-container {
			padding-top: 10px;
		}

		.featured-badge {
			left: 14px;
			padding: 6px 12px;
			gap: 5px;
		}

		.videos-layout {
			gap: 28px;
		}

		.empty-state {
			padding: 56px 24px;
			border-radius: 16px;
		}

		.empty-icon {
			width: 64px;
			height: 64px;
			border-radius: 16px;
			margin-bottom: 20px;
		}

		.empty-icon svg {
			width: 32px;
			height: 32px;
		}

		.empty-title {
			font-size: 17px;
		}

		.empty-text {
			font-size: 14px;
		}
	}

	/* sm: 640px+ - 2 column grid */
	@media (min-width: 640px) {
		.grid-container {
			grid-template-columns: repeat(2, 1fr);
			gap: 16px;
			padding: 0 16px;
		}

		.skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 16px;
		}

		.videos-layout {
			gap: 32px;
		}

		.featured-badge {
			font-size: 11px;
			padding: 7px 13px;
			gap: 6px;
			border-radius: 7px;
		}

		.badge-icon {
			width: 13px;
			height: 13px;
		}

		.view-more-btn {
			padding: 14px 28px;
			font-size: 14px;
			border-radius: 12px;
		}

		.view-more-btn:hover {
			transform: translateY(-2px);
			box-shadow: 0 8px 24px rgba(20, 62, 89, 0.35);
			background: linear-gradient(135deg, #1a4d6e 0%, #246089 100%);
		}

		.view-more-btn:hover .view-more-icon {
			transform: translateX(4px);
		}

		.view-more-icon {
			width: 18px;
			height: 18px;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.grid-container {
			gap: 20px;
			padding: 0 20px;
		}

		.skeleton-grid {
			gap: 20px;
		}

		.videos-layout {
			gap: 36px;
		}
	}

	/* lg: 1024px+ - 3 column grid */
	@media (min-width: 1024px) {
		.grid-container {
			grid-template-columns: repeat(3, 1fr);
			gap: 24px;
			padding: 0 24px;
		}

		.skeleton-grid {
			grid-template-columns: repeat(3, 1fr);
			gap: 24px;
		}

		.videos-layout {
			gap: 40px;
		}

		.featured-container {
			max-width: 880px;
			padding-top: 12px;
		}

		.featured-badge {
			left: 20px;
			font-size: 11px;
			padding: 8px 14px;
			border-radius: 8px;
		}

		.badge-icon {
			width: 14px;
			height: 14px;
		}

		.empty-state {
			padding: 72px 32px;
			border-radius: 20px;
		}

		.empty-icon {
			width: 72px;
			height: 72px;
			border-radius: 18px;
			margin-bottom: 24px;
		}

		.empty-icon svg {
			width: 36px;
			height: 36px;
		}

		.empty-title {
			font-size: 18px;
		}

		.empty-text {
			font-size: 15px;
		}
	}

	/* xl: 1280px+ - 4 column grid option (keeping 3 for design consistency) */
	@media (min-width: 1280px) {
		.featured-container {
			max-width: 960px;
		}

		.grid-container {
			gap: 28px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	   ACCESSIBILITY
	   ═══════════════════════════════════════════════════════════════════════════ */

	@media (prefers-reduced-motion: reduce) {
		.view-more-btn,
		.view-more-icon {
			transition: none;
		}

		.view-more-btn:hover {
			transform: none;
		}

		@keyframes shimmer {
			0%,
			100% {
				background-position: 0 0;
			}
		}
	}

	.video-card-btn:focus-visible,
	.featured-card-wrapper:focus-visible,
	.view-more-btn:focus-visible {
		outline: 2px solid var(--color-brand-primary, #0984ae);
		outline-offset: 2px;
	}
</style>
