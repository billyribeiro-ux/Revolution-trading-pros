<!--
	Training Videos Section Component
	═══════════════════════════════════════════════════════════════════════════
	Displays training videos for indicators
	Used across all indicator detail pages
-->
<script lang="ts">
	interface TrainingVideo {
		id: string;
		title: string;
		videoUrl: string;
		posterUrl: string;
	}

	interface Props {
		videos: TrainingVideo[];
	}

	let { videos }: Props = $props();
</script>

<section id="ca-main" class="ca-section cpost-section">
	<div class="section-inner">
		<div class="ca-content-block cpost-content-block">
			{#each videos as video (video.id)}
				<div class="current-vid">
					<div class="video-container current">
						<div class="video-overlay"></div>
						<div id={video.id} class="video-player"></div>
						<video
							id={video.id}
							controls
							width="100%"
							poster={video.posterUrl}
							style="aspect-ratio: 16/9;"
							title={video.title}
						>
							<source src={video.videoUrl} type="video/mp4" />
							Your browser does not support the video tag.
						</video>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Training Videos Section - 2026 Mobile-First Responsive Design
	 * Features: responsive video, safe areas, touch-friendly controls
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile-first base styles */
	.ca-section {
		padding-bottom: 0;
		padding-bottom: max(0px, env(safe-area-inset-bottom));
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0;
		padding-left: max(0px, env(safe-area-inset-left));
		padding-right: max(0px, env(safe-area-inset-right));
	}

	.ca-content-block {
		margin: 0;
		padding: 0;
	}

	.current-vid {
		width: 100%;
		background-color: #f4f4f4;
		/* Mobile-first padding */
		padding: 8px 8px 0;
		box-sizing: border-box;
	}

	.current-vid:last-child {
		padding-bottom: 16px;
	}

	.video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid #999;
		border-radius: 4px;
		cursor: pointer;
		/* Ensure aspect ratio on all devices */
		aspect-ratio: 16 / 9;
	}

	.video-container.current {
		width: 100%;
		display: flex;
		z-index: 1;
	}

	.video-container video {
		width: 100%;
		height: 100%;
		display: block;
		object-fit: contain;
		/* Better video rendering */
		-webkit-transform: translateZ(0);
		transform: translateZ(0);
	}

	.video-overlay {
		background-color: rgba(0, 0, 0, 0.269);
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	/* xs: Extra small devices (≥ 360px) */
	@media (min-width: 360px) {
		.current-vid {
			padding: 10px 10px 0;
		}

		.current-vid:last-child {
			padding-bottom: 20px;
		}
	}

	/* sm: Small devices (≥ 640px) */
	@media (min-width: 640px) {
		.current-vid {
			padding: 16px 16px 0;
		}

		.current-vid:last-child {
			padding-bottom: 24px;
		}

		.video-container {
			border-radius: 6px;
		}
	}

	/* md: Medium devices (≥ 768px) */
	@media (min-width: 768px) {
		.current-vid {
			padding: 20px 20px 0;
		}

		.current-vid:last-child {
			padding-bottom: 25px;
		}

		.video-container {
			border-radius: 8px;
		}
	}

	/* lg: Large devices (≥ 1024px) */
	@media (min-width: 1024px) {
		.current-vid {
			padding: 25px 25px 0;
		}

		.video-container:hover {
			box-shadow: 0 4px 16px rgba(0, 0, 0, 0.2);
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.video-container {
			/* Remove hover effects on touch devices */
			cursor: default;
		}

		/* Ensure touch-friendly video controls */
		.video-container video::-webkit-media-controls-panel {
			min-height: 44px;
		}

		.video-container video::-webkit-media-controls-play-button {
			width: 44px;
			height: 44px;
		}

		.video-container video::-webkit-media-controls-timeline {
			height: 44px;
		}

		.video-container video::-webkit-media-controls-volume-slider {
			height: 44px;
		}
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.video-container {
			transition: none;
		}

		.video-container:hover {
			box-shadow: none;
		}
	}

	/* Landscape mode on mobile */
	@media (max-height: 500px) and (orientation: landscape) {
		.current-vid {
			padding: 8px;
		}

		.current-vid:last-child {
			padding-bottom: 8px;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.video-container {
			border-width: 2px;
			border-color: #000;
		}
	}

	/* Dark mode support */
	@media (prefers-color-scheme: dark) {
		.current-vid {
			background-color: #1a1a1a;
		}

		.video-container {
			border-color: #444;
		}
	}
</style>
