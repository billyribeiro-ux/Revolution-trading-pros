<script lang="ts">
	/**
	 * ═══════════════════════════════════════════════════════════════════════════════
	 * VideoGrid Component - Latest Video Updates Grid
	 * ═══════════════════════════════════════════════════════════════════════════════
	 *
	 * @description Grid display for recent videos with featured video
	 * @version 4.0.0 - January 2026 - Nuclear Build Specification
	 * @standards Apple Principal Engineer ICT 7+ Standards
	 */
	import type { Video } from '../types';
	import VideoCard from './VideoCard.svelte';

	interface Props {
		videos: Video[];
		isLoading?: boolean;
	}

	const { videos, isLoading = false }: Props = $props();

	const featuredVideo = $derived(videos.find((v) => v.isFeatured) || videos[0]);
	const gridVideos = $derived(videos.filter((v) => v !== featuredVideo).slice(0, 5));
</script>

<section class="video-grid-section" aria-labelledby="videos-heading">
	<div class="section-header">
		<h2 id="videos-heading" class="section-title">Latest Updates</h2>
		<p class="section-subtitle">Video breakdowns as we enter and exit trades</p>
	</div>

	{#if isLoading}
		<div class="loading-grid">
			<div class="skeleton-featured"></div>
			<div class="skeleton-grid">
				{#each Array(3) as _}
					<div class="skeleton-card"></div>
				{/each}
			</div>
		</div>
	{:else if videos.length === 0}
		<div class="empty-state">
			<p>No video updates yet. Check back soon!</p>
		</div>
	{:else}
		<div class="videos-layout">
			<!-- Featured Video (2x size) -->
			{#if featuredVideo}
				<div class="featured-video">
					<VideoCard video={featuredVideo} variant="featured" />
				</div>
			{/if}

			<!-- Grid Videos -->
			{#if gridVideos.length > 0}
				<div class="grid-videos">
					{#each gridVideos as video (video.id)}
						<VideoCard {video} variant="default" />
					{/each}
				</div>
			{/if}
		</div>
	{/if}
</section>

<style>
	.video-grid-section {
		margin-top: 40px;
		padding-top: 40px;
		border-top: 1px solid #e2e8f0;
	}

	.section-header {
		margin-bottom: 24px;
	}

	.section-title {
		font-size: 20px;
		font-weight: 600;
		color: #0f172a;
		margin: 0 0 4px 0;
	}

	.section-subtitle {
		font-size: 14px;
		color: #64748b;
		margin: 0;
	}

	.videos-layout {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.featured-video {
		width: 100%;
	}

	.grid-videos {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
	}

	.loading-grid {
		display: flex;
		flex-direction: column;
		gap: 24px;
	}

	.skeleton-featured {
		width: 100%;
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 12px;
	}

	.skeleton-grid {
		display: grid;
		grid-template-columns: repeat(3, 1fr);
		gap: 20px;
	}

	.skeleton-card {
		aspect-ratio: 16 / 9;
		background: linear-gradient(90deg, #f1f5f9 25%, #e2e8f0 50%, #f1f5f9 75%);
		background-size: 200% 100%;
		animation: shimmer 1.5s infinite;
		border-radius: 12px;
	}

	@keyframes shimmer {
		0% {
			background-position: 200% 0;
		}
		100% {
			background-position: -200% 0;
		}
	}

	.empty-state {
		text-align: center;
		padding: 48px 24px;
		color: #64748b;
		font-size: 14px;
	}

	@media (max-width: 1024px) {
		.grid-videos {
			grid-template-columns: repeat(2, 1fr);
		}

		.skeleton-grid {
			grid-template-columns: repeat(2, 1fr);
		}
	}

	@media (max-width: 640px) {
		.grid-videos {
			grid-template-columns: 1fr;
		}

		.skeleton-grid {
			grid-template-columns: 1fr;
		}
	}
</style>
