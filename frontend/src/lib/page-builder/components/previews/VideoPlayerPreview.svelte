<script lang="ts">
	/**
	 * Video Player Preview Component
	 * Apple Principal Engineer ICT 7 Grade - January 2026
	 */

	import type { VideoPlayerConfig } from '../../types';

	interface Props {
		config: VideoPlayerConfig;
		isPreview: boolean;
	}

	let { config, isPreview }: Props = $props();

	const hasVideo = $derived(!!config.bunnyVideoGuid || !!config.videoId);
</script>

<div class="video-player">
	<div class="video-header">
		<h3 class="video-title">{config.title || 'Video Title'}</h3>
		{#if config.subtitle}
			<span class="video-subtitle">{config.subtitle}</span>
		{/if}
	</div>
	<div class="video-container">
		{#if hasVideo && isPreview && config.bunnyVideoGuid}
			<iframe
				src="https://iframe.mediadelivery.net/embed/{config.bunnyLibraryId ?? 0}/{config.bunnyVideoGuid}?autoplay=false"
				loading="lazy"
				style="border: none; width: 100%; height: 100%;"
				allow="accelerometer; gyroscope; autoplay; encrypted-media; picture-in-picture;"
				allowfullscreen
				title={config.title}
			></iframe>
		{:else}
			<div class="video-placeholder">
				{#if config.thumbnailUrl}
					<img src={config.thumbnailUrl} alt={config.title} class="thumbnail" />
				{:else}
					<div class="placeholder-content">
						<svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
							<polygon points="5 3 19 12 5 21 5 3"/>
						</svg>
						<span>{hasVideo ? 'Video ready' : 'No video uploaded'}</span>
					</div>
				{/if}
				{#if config.thumbnailUrl}
					<div class="play-overlay">
						<div class="play-button">
							<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="white">
								<polygon points="5 3 19 12 5 21 5 3"/>
							</svg>
						</div>
					</div>
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.video-player {
		border: 1px solid #E5E7EB;
		border-radius: 4px;
		overflow: hidden;
	}

	.video-header {
		background: #1F2937;
		padding: 12px 16px;
		display: flex;
		align-items: center;
		justify-content: space-between;
	}

	.video-title {
		color: white;
		font-size: 14px;
		font-weight: 600;
		margin: 0;
	}

	.video-subtitle {
		color: rgba(255, 255, 255, 0.7);
		font-size: 12px;
	}

	.video-container {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		background: #000;
	}

	.video-container iframe {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
	}

	.video-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1F2937;
	}

	.thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
	}

	.placeholder-content {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 12px;
		color: #6B7280;
	}

	.play-overlay {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
	}

	.play-button {
		width: 64px;
		height: 64px;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		padding-left: 4px;
		cursor: pointer;
		transition: transform 0.2s ease, background 0.2s ease;
	}

	.play-button:hover {
		transform: scale(1.1);
		background: rgba(0, 0, 0, 0.9);
	}
</style>
