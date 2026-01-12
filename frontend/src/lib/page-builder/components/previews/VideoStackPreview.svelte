<script lang="ts">
	/**
	 * Video Stack Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 * 
	 * Displays multiple videos stacked vertically - matches the WordPress reference.
	 */

	import type { VideoStackConfig } from '../../types';
	import VideoPlayerPreview from './VideoPlayerPreview.svelte';

	interface Props {
		config: VideoStackConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();

	const sortedVideos = $derived(() => {
		if (!config.videos || config.videos.length === 0) return [];
		
		const videos = [...config.videos];
		if (config.sortOrder === 'newest') {
			return videos.reverse();
		}
		return videos;
	});
</script>

<div class="video-stack">
	{#if config.videos && config.videos.length > 0}
		{#each sortedVideos() as video, index}
			<div class="video-item">
				<VideoPlayerPreview config={video} {isPreview} />
			</div>
		{/each}
	{:else}
		<div class="empty-stack">
			<div class="empty-content">
				<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
					<rect width="18" height="18" x="3" y="3" rx="2" ry="2"/>
					<line x1="9" x2="15" y1="9" y2="9"/>
					<line x1="9" x2="15" y1="15" y2="15"/>
				</svg>
				<p>No videos added yet</p>
				<span>Add videos in the configuration panel</span>
			</div>
		</div>
	{/if}
</div>

<style>
	.video-stack {
		display: flex;
		flex-direction: column;
	}

	.video-item {
		border-bottom: 1px solid #E5E7EB;
	}

	.video-item:last-child {
		border-bottom: none;
	}

	.empty-stack {
		border: 2px dashed #E5E7EB;
		border-radius: 8px;
		padding: 48px;
	}

	.empty-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		text-align: center;
		color: #9CA3AF;
	}

	.empty-content p {
		margin: 16px 0 4px 0;
		font-weight: 500;
		color: #6B7280;
	}

	.empty-content span {
		font-size: 13px;
	}
</style>
