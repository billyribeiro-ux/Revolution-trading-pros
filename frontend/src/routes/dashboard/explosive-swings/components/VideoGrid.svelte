<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoGrid Component - Latest Video Updates Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Grid display for recent videos with featured video
	 * @version 4.1.0 - Visual Polish Pass
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';
	import VideoCard from './VideoCard.svelte';

	interface Props {
		videos: Video[];
		isLoading?: boolean;
		title?: string;
		subtitle?: string;
	}

	const { 
		videos, 
		isLoading = false,
		title = 'Latest Updates',
		subtitle = 'Video breakdowns as we enter and exit trades'
	}: Props = $props();

	const featuredVideo = $derived(videos.find((v) => v.isFeatured) || videos[0]);
	const gridVideos = $derived(videos.filter((v) => v !== featuredVideo).slice(0, 5));
</script>

<section class="video-grid-section" aria-labelledby="videos-heading">
	<!-- Section Header -->
	<header class="section-header">
		<div class="header-content">
			<h2 id="videos-heading" class="section-title">{title}</h2>
			<p class="section-subtitle">{subtitle}</p>
		</div>
		<a href="/dashboard/explosive-swings/video-library" class="view-all-link">
			View All
			<svg viewBox="0 0 20 20" fill="currentColor" class="arrow-icon">
				<path fill-rule="evenodd" d="M3 10a.75.75 0 01.75-.75h10.638L10.23 5.29a.75.75 0 111.04-1.08l5.5 5.25a.75.75 0 010 1.08l-5.5 5.25a.75.75 0 11-1.04-1.08l4.158-3.96H3.75A.75.75 0 013 10z" clip-rule="evenodd" />
			</svg>
		</a>
	</header>

	{#if isLoading}
		<!-- Loading State -->
		<div class="loading-container">
			<div class="skeleton-featured">
				<div class="skeleton-play"></div>
			</div>
			<div class="skeleton-grid">
				{#each Array(3) as _, i}
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
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<path stroke-linecap="round" stroke-linejoin="round" d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
					<path stroke-linecap="round" stroke-linejoin="round" d="M15.91 11.672a.375.375 0 010 .656l-5.603 3.113a.375.375 0 01-.557-.328V8.887c0-.286.307-.466.557-.327l5.603 3.112z" />
				</svg>
			</div>
			<h3 class="empty-title">No video updates yet</h3>
			<p class="empty-text">Check back soon for trade breakdowns and analysis videos.</p>
		</div>
	{:else}
		<!-- Videos Layout -->
		<div class="videos-layout">
			<!-- Featured Video (Larger) -->
			{#if featuredVideo}
				<div class="featured-wrapper">
					<span class="featured-label">
						<svg viewBox="0 0 20 20" fill="currentColor" class="star-icon">
							<path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z" clip-rule="evenodd" />
						</svg>
						Featured
					</span>
					<VideoCard video={featuredVideo} variant="featured" />
				</div>
			{/if}

			<!-- Grid Videos -->
			{#if gridVideos.length > 0}
				<div class="grid-wrapper">
					{#each gridVideos as video (video.id)}
						<VideoCard {video} variant="default" />
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════
	   SECTION CONTAINER
	   ═══════════════════════════════════════════════════════════════════════ */
	.video-grid-section {
		margin-top: 48px;
		padding: 32px 0 0;
		border-top: 1px solid #e2e8f0;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   HEADER
	   ═══════════════════════════════════════════════════════════════════════ */
	.section-header {
		display: flex;
		align-items: flex-start;
		justify-content: space-between;
		margin-bottom: 28px;
		flex-wrap: wrap;
		gap: 16px;
	}

	.header-content {
		display: flex;
		flex-direction: column;
		gap: 4px;
	}

	.section-title {
		font-size: 22px;
		font-weight: 700;
		color: #0f172a;
		margin: 0;
		letter-spacing: -0.01em;
	}

	.section-subtitle {
		font-size: 14px;
		color: #64748b;
		margin: 0;
	}

	.view-all-link {
		display: inline-flex;
		align-items: center;
		gap: 6px;
		font-size: 14px;
		font-weight: 600;
		color: #143e59;
		text-decoration: none;
		padding: 8px 16px;
		border-radius: 8px;
		transition: all 0.2s ease-out;
	}

	.view-all-link:hover {
		background: #f1f5f9;
		color: #0f172a;
	}

	.view-all-link:hover .arrow-icon {
		transform: translateX(3px);
	}

	.arrow-icon {
		width: 16px;
		height: 16px;
		transition: transform 0.2s ease-out;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   VIDEOS LAYOUT
	   ═══════════════════════════════════════════════════════════════════════ */
	.videos-layout {
		display: flex;
		flex-direction: column;
		gap: 28px;
	}

	/* Featured Video Wrapper */
	.featured-wrapper {
		position: relative;
	}

	.featured-label {
		position: absolute;
		top: -12px;
		left: 20px;
		z-index: 10;
		display: inline-flex;
		align-items: center;
		gap: 5px;
		padding: 6px 12px;
		background: linear-gradient(135deg, #f69532 0%, #e8860d 100%);
		color: #fff;
		font-size: 11px;
		font-weight: 700;
		text-transform: uppercase;
		letter-spacing: 0.05em;
		border-radius: 6px;
		box-shadow: 0 2px 8px rgba(246, 149, 50, 0.4);
	}

	.star-icon {
		width: 14px;
		height: 14px;
	}

	/* Video Grid */
	.grid-wrapper {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 24px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   LOADING STATE
	   ═══════════════════════════════════════════════════════════════════════ */
	.loading-container {
		display: flex;
		flex-direction: column;
		gap: 28px;
	}

	.skeleton-featured {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
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
		background: rgba(255, 255, 255, 0.3);
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
		background: #fff;
		border: 1px solid #e2e8f0;
	}

	.skeleton-thumb {
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
	}

	.skeleton-info {
		padding: 16px;
	}

	.skeleton-title {
		height: 18px;
		width: 85%;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 4px;
		margin-bottom: 10px;
	}

	.skeleton-date {
		height: 14px;
		width: 50%;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
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
		padding: 64px 24px;
		text-align: center;
		background: #f8fafc;
		border: 2px dashed #e2e8f0;
		border-radius: 16px;
	}

	.empty-icon {
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e2e8f0;
		border-radius: 16px;
		margin-bottom: 20px;
	}

	.empty-icon svg {
		width: 32px;
		height: 32px;
		color: #94a3b8;
	}

	.empty-title {
		font-size: 17px;
		font-weight: 600;
		color: #334155;
		margin: 0 0 8px 0;
	}

	.empty-text {
		font-size: 14px;
		color: #64748b;
		margin: 0;
		max-width: 300px;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.grid-wrapper {
			grid-template-columns: repeat(2, 1fr);
		}

		.skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.video-grid-section {
			margin-top: 32px;
			padding-top: 24px;
		}

		.section-header {
			flex-direction: column;
			align-items: flex-start;
		}

		.section-title {
			font-size: 18px;
		}

		.grid-wrapper {
			grid-template-columns: 1fr;
			gap: 16px;
		}

		.skeleton-grid {
			grid-template-columns: 1fr;
		}

		.featured-label {
			top: -10px;
			left: 14px;
			font-size: 10px;
			padding: 5px 10px;
		}
	}
</style>