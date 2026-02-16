<script lang="ts">
	/**
	 * VideoPlayer Component - Apple Principal Engineer ICT Level 7
	 * Extracted from WeeklyHero.svelte for better maintainability
	 * 
	 * @description Video player with thumbnail, play button, iframe, and controls
	 * @version 1.0.0 - ICT 7 compliance
	 */

	interface Props {
		thumbnail: string;
		videoTitle: string;
		duration: string;
		embedUrl: string;
		isPlaying?: boolean;
		isExpanded?: boolean;
		onPlay?: () => void;
		onClose?: () => void;
		onToggleExpand?: () => void;
	}

	const {
		thumbnail,
		videoTitle,
		duration,
		embedUrl,
		isPlaying = false,
		isExpanded = false,
		onPlay,
		onClose,
		onToggleExpand
	}: Props = $props();
</script>

<div
	class="video-player-container"
	class:playing={isPlaying}
	class:expanded={isExpanded}
>
	{#if isPlaying}
		<!-- Active Video Player with iframe -->
		<div class="video-frame">
			{#if embedUrl}
				<iframe
					src={embedUrl}
					title={videoTitle}
					frameborder="0"
					allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; fullscreen"
				></iframe>
			{:else}
				<div class="video-error">
					<p>Video unavailable</p>
				</div>
			{/if}
		</div>

		<!-- Video Controls Bar -->
		<div class="video-controls">
			<button
				class="control-btn close-btn"
				onclick={onClose}
				type="button"
				aria-label="Close video"
			>
				<svg
					viewBox="0 0 24 24"
					fill="none"
					stroke="currentColor"
					stroke-width="2"
					width="20"
					height="20"
					aria-hidden="true"
				>
					<path d="M18 6L6 18M6 6l12 12" />
				</svg>
			</button>
			<span class="video-title-bar">{videoTitle}</span>
			<button
				class="control-btn expand-btn"
				onclick={onToggleExpand}
				type="button"
				aria-label={isExpanded ? 'Exit fullscreen' : 'Enter fullscreen'}
			>
				{#if isExpanded}
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="20"
						height="20"
						aria-hidden="true"
					>
						<path
							d="M8 3v3a2 2 0 01-2 2H3m18 0h-3a2 2 0 01-2-2V3m0 18v-3a2 2 0 012-2h3M3 16h3a2 2 0 012 2v3"
						/>
					</svg>
				{:else}
					<svg
						viewBox="0 0 24 24"
						fill="none"
						stroke="currentColor"
						stroke-width="2"
						width="20"
						height="20"
						aria-hidden="true"
					>
						<path
							d="M8 3H5a2 2 0 00-2 2v3m18 0V5a2 2 0 00-2-2h-3m0 18h3a2 2 0 002-2v-3M3 16v3a2 2 0 002 2h3"
						/>
					</svg>
				{/if}
			</button>
		</div>
	{:else}
		<!-- Thumbnail State with Play Button -->
		<div class="video-thumbnail" style="background-image: url('{thumbnail}')">
			<button
				class="play-btn"
				onclick={onPlay}
				type="button"
				aria-label="Play video: {videoTitle}"
			>
				<svg viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
					<path d="M8 5v14l11-7z" />
				</svg>
			</button>
			<span class="duration-badge" aria-label="Duration: {duration}">
				{duration}
			</span>
		</div>
	{/if}
</div>

<style>
	/* Video Player Container */
	.video-player-container {
		position: relative;
		width: 100%;
		aspect-ratio: 16 / 9;
		background: #000;
		border-radius: 12px;
		overflow: hidden;
		transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
	}

	.video-player-container.expanded {
		position: fixed;
		inset: 0;
		z-index: 9999;
		border-radius: 0;
		width: 100vw;
		height: 100vh;
		aspect-ratio: auto;
	}

	/* Video Thumbnail */
	.video-thumbnail {
		position: absolute;
		inset: 0;
		background-size: cover;
		background-position: center;
		background-repeat: no-repeat;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	/* Play Button - 80px frosted glass */
	.play-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 80px;
		height: 80px;
		background: rgba(255, 255, 255, 0.25);
		backdrop-filter: blur(20px) saturate(180%);
		-webkit-backdrop-filter: blur(20px) saturate(180%);
		border: 2px solid rgba(255, 255, 255, 0.3);
		border-radius: 50%;
		cursor: pointer;
		transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		z-index: 10;
	}

	.play-btn:hover {
		background: rgba(255, 255, 255, 0.35);
		border-color: rgba(255, 255, 255, 0.5);
		transform: scale(1.1);
		box-shadow: 0 12px 40px rgba(0, 0, 0, 0.4);
	}

	.play-btn:active {
		transform: scale(0.95);
	}

	.play-btn:focus-visible {
		outline: 3px solid var(--color-bg-card);
		outline-offset: 4px;
	}

	.play-btn svg {
		width: 28px;
		height: 28px;
		color: var(--color-bg-card);
		margin-left: 4px;
		filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
		transition: all 0.3s ease;
	}

	.play-btn:hover svg {
		transform: scale(1.05);
		filter: drop-shadow(0 4px 12px rgba(0, 0, 0, 0.4));
	}

	/* Duration Badge */
	.duration-badge {
		position: absolute;
		bottom: 12px;
		right: 12px;
		padding: 6px 12px;
		background: rgba(0, 0, 0, 0.8);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		border-radius: 6px;
		font-size: 13px;
		font-weight: 600;
		font-variant-numeric: tabular-nums;
		color: var(--color-bg-card);
		z-index: 5;
	}

	/* Video Frame (iframe container) */
	.video-frame {
		position: absolute;
		inset: 0;
	}

	.video-frame iframe {
		width: 100%;
		height: 100%;
		border: none;
	}

	.video-error {
		position: absolute;
		inset: 0;
		display: flex;
		align-items: center;
		justify-content: center;
		background: #1a1a1a;
		color: #888;
		font-size: 14px;
	}

	.video-error p {
		margin: 0;
	}

	/* Video Controls - Top Bar */
	.video-controls {
		position: absolute;
		top: 0;
		left: 0;
		right: 0;
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		background: linear-gradient(
			to bottom,
			rgba(0, 0, 0, 0.8) 0%,
			rgba(0, 0, 0, 0.4) 60%,
			transparent 100%
		);
		z-index: 20;
		opacity: 0;
		transition: opacity 0.25s ease;
	}

	.video-player-container:hover .video-controls {
		opacity: 1;
	}

	.video-player-container.expanded .video-controls {
		opacity: 1;
		padding: 20px 24px;
	}

	.control-btn {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: rgba(255, 255, 255, 0.2);
		backdrop-filter: blur(8px);
		-webkit-backdrop-filter: blur(8px);
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 10px;
		color: var(--color-bg-card);
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.control-btn:hover {
		background: rgba(255, 255, 255, 0.35);
		transform: scale(1.05);
		border-color: rgba(255, 255, 255, 0.2);
	}

	.control-btn:active {
		transform: scale(0.95);
	}

	.control-btn:focus-visible {
		outline: 2px solid var(--color-bg-card);
		outline-offset: 2px;
	}

	.video-title-bar {
		flex: 1;
		text-align: center;
		color: var(--color-bg-card);
		font-size: 14px;
		font-weight: 600;
		padding: 0 16px;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
		text-shadow: 0 1px 2px rgba(0, 0, 0, 0.5);
	}

	/* Responsive Breakpoints */
	@media (min-width: 768px) {
		.play-btn {
			width: 80px;
			height: 80px;
		}

		.play-btn svg {
			width: 36px;
			height: 36px;
		}
	}

	@media (min-width: 1920px) {
		.play-btn {
			width: 100px;
			height: 100px;
		}

		.play-btn svg {
			width: 44px;
			height: 44px;
		}
	}

	@media (min-width: 2560px) {
		.play-btn {
			width: 120px;
			height: 120px;
		}

		.play-btn svg {
			width: 52px;
			height: 52px;
		}
	}

	/* Reduced Motion */
	@media (prefers-reduced-motion: reduce) {
		.video-player-container,
		.play-btn,
		.control-btn {
			transition: none;
		}

		.play-btn:hover {
			transform: none;
		}
	}
</style>

