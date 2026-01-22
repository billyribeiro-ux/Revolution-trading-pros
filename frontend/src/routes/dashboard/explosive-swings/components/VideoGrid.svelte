<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoGrid Component - Latest Video Updates Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Grid display for recent videos with featured video
	 * @version 4.2.0 - Professional Polish Refinements
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
		title = '',
		subtitle = ''
	}: Props = $props();

	const featuredVideo = $derived(videos.find((v) => v.isFeatured) || videos[0]);
	const gridVideos = $derived(videos.filter((v) => v !== featuredVideo).slice(0, 3));
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
				<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true">
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
			<!-- Featured Video -->
			{#if featuredVideo}
				<div class="featured-container">
					<div class="featured-badge">
						<svg viewBox="0 0 20 20" fill="currentColor" class="badge-icon" aria-hidden="true">
							<path fill-rule="evenodd" d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401z" clip-rule="evenodd" />
						</svg>
						<span>Featured</span>
					</div>
					<div class="featured-card-wrapper">
						<VideoCard video={featuredVideo} variant="featured" />
					</div>
				</div>
			{/if}

			<!-- Grid Videos -->
			{#if gridVideos.length > 0}
				<div class="grid-container">
					{#each gridVideos as video (video.id)}
						<VideoCard {video} variant="default" />
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</div>

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
		background: linear-gradient(135deg, #f69532 0%, #e07d0a 100%);
		color: #fff;
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
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
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
		background: #fff;
		border: 1px solid #e2e8f0;
	}

	.skeleton-thumb {
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
	}

	.skeleton-info {
		padding: 16px;
	}

	.skeleton-title {
		height: 18px;
		width: 85%;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite ease-in-out;
		border-radius: 6px;
		margin-bottom: 12px;
	}

	.skeleton-date {
		height: 14px;
		width: 45%;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
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
		background: linear-gradient(180deg, #f8fafc 0%, #f1f5f9 100%);
		border: 1px solid #e2e8f0;
		border-radius: 20px;
	}

	.empty-icon {
		width: 72px;
		height: 72px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #e2e8f0;
		border-radius: 18px;
		margin-bottom: 24px;
	}

	.empty-icon svg {
		width: 36px;
		height: 36px;
		color: #94a3b8;
	}

	.empty-title {
		font-size: 18px;
		font-weight: 600;
		color: #334155;
		margin: 0 0 8px 0;
		letter-spacing: -0.01em;
	}

	.empty-text {
		font-size: 15px;
		color: #64748b;
		margin: 0;
		max-width: 320px;
		line-height: 1.5;
	}

	/* ═══════════════════════════════════════════════════════════════════════
	   RESPONSIVE BREAKPOINTS
	   ═══════════════════════════════════════════════════════════════════════ */
	@media (max-width: 1024px) {
		.featured-container {
			max-width: 100%;
		}

		.grid-container {
			grid-template-columns: repeat(2, 1fr);
			gap: 20px;
			padding: 0 20px;
		}

		.skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
			gap: 20px;
		}
	}

	@media (max-width: 768px) {
		.videos-layout {
			gap: 32px;
		}

		.grid-container {
			gap: 16px;
			padding: 0 16px;
		}
	}

	@media (max-width: 640px) {
		.videos-layout {
			gap: 28px;
		}

		.featured-container {
			padding-top: 10px;
		}

		.featured-badge {
			left: 14px;
			font-size: 10px;
			padding: 6px 12px;
			gap: 5px;
			border-radius: 6px;
		}

		.badge-icon {
			width: 12px;
			height: 12px;
		}

		.grid-container {
			grid-template-columns: 1fr;
			gap: 14px;
			padding: 0 14px;
		}

		.skeleton-grid {
			grid-template-columns: 1fr;
			gap: 14px;
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
</style>