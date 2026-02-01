<!--
	Class Video Section Component - SSOT
	═══════════════════════════════════════════════════════════════════════════
	Standardized video player section for all class pages.
	This is the Single Source of Truth (SSOT) for class video sections.
	
	@version 1.0.0 - ICT 7 Standards (Nov 2025)
	@author Revolution Trading Pros
-->
<script lang="ts">
	// ICT 7 FIX: Svelte 5 $props() syntax (Nov 2025 best practice)
	interface Props {
		videoUrl: string;
		videoTitle: string;
		videoId?: string;
		overlayTitle?: string;
		overlayDescription?: string;
		showOverlay?: boolean;
		poster?: string;
	}

	let {
		videoUrl,
		videoTitle,
		videoId = '',
		overlayTitle = '',
		overlayDescription = '',
		showOverlay = false,
		poster = ''
	}: Props = $props();
</script>

<div class="class-video-container current current-vid">
	{#if showOverlay && overlayTitle}
		<div class="video-overlay">
			<h3>{overlayTitle}</h3>
			{#if overlayDescription}
				<div class="class-video-description">
					<p>{overlayDescription}</p>
				</div>
			{/if}
		</div>
	{/if}

	{#if videoId}
		<div id={videoId} class="class-video-player"></div>
	{/if}

	<video
		id={videoUrl}
		controls
		width="100%"
		{poster}
		style="aspect-ratio: 16/9;"
		title={videoTitle}
	>
		<source src={videoUrl} type="video/mp4" />
		Your browser does not support the video tag.
	</video>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Class Video Section - Apple ICT 7 Standards
	 * SSOT for all class video sections across the platform
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-video-container {
		width: 100%;
		margin-bottom: 30px;
		background: #000000;
		border-radius: 4px;
		overflow: hidden;
		position: relative;
	}

	.video-overlay {
		padding: 15px 20px;
		background-color: #f5f5f5;
		border-bottom: 1px solid #e0e0e0;
	}

	.video-overlay h3 {
		font-size: 1.2rem;
		font-weight: 600;
		color: #333333;
		margin: 0 0 10px 0;
	}

	.class-video-description {
		margin-top: 10px;
	}

	.class-video-description p {
		font-size: 1rem;
		line-height: 1.6;
		color: #666666;
		margin: 0;
	}

	.class-video-player {
		width: 100%;
		min-height: 50px;
	}

	.class-video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * 2026 MOBILE-FIRST RESPONSIVE DESIGN
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Safe areas: env(safe-area-inset-*) for fullscreen viewing
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile base styles (default) */
	.class-video-container {
		margin-bottom: 16px;
		border-radius: 6px;
		/* Safe area support */
		padding-left: env(safe-area-inset-left, 0);
		padding-right: env(safe-area-inset-right, 0);
	}

	.video-overlay {
		padding: 10px 12px;
	}

	.video-overlay h3 {
		font-size: 0.95rem;
		margin-bottom: 6px;
	}

	.class-video-description p {
		font-size: 0.85rem;
		line-height: 1.5;
	}

	/* Video element with aspect-ratio */
	.class-video-container video {
		aspect-ratio: 16 / 9;
		width: 100%;
		height: auto;
	}

	/* Fallback for browsers without aspect-ratio */
	@supports not (aspect-ratio: 16 / 9) {
		.class-video-container video {
			padding-top: 56.25%;
			height: 0;
			position: relative;
		}
	}

	/* xs: 360px+ */
	@media (min-width: 360px) {
		.class-video-container {
			margin-bottom: 18px;
		}

		.video-overlay {
			padding: 12px 14px;
		}

		.video-overlay h3 {
			font-size: 1rem;
			margin-bottom: 8px;
		}

		.class-video-description p {
			font-size: 0.9rem;
		}
	}

	/* sm: 640px+ */
	@media (min-width: 640px) {
		.class-video-container {
			margin-bottom: 24px;
			border-radius: 6px;
		}

		.video-overlay {
			padding: 14px 18px;
		}

		.video-overlay h3 {
			font-size: 1.1rem;
			margin-bottom: 10px;
		}

		.class-video-description p {
			font-size: 0.95rem;
			line-height: 1.6;
		}
	}

	/* md: 768px+ */
	@media (min-width: 768px) {
		.class-video-container {
			margin-bottom: 28px;
		}

		.video-overlay {
			padding: 15px 20px;
		}

		.video-overlay h3 {
			font-size: 1.2rem;
		}

		.class-video-description p {
			font-size: 1rem;
		}
	}

	/* lg: 1024px+ */
	@media (min-width: 1024px) {
		.class-video-container {
			margin-bottom: 30px;
			border-radius: 8px;
		}
	}

	/* Touch device optimizations */
	@media (hover: none) and (pointer: coarse) {
		.class-video-container video::-webkit-media-controls-panel {
			min-height: 44px;
		}

		.class-video-container video::-webkit-media-controls-play-button {
			width: 44px;
			height: 44px;
		}

		.class-video-container video::-webkit-media-controls-timeline {
			height: 44px;
		}

		.class-video-container video::-webkit-media-controls-volume-slider {
			height: 44px;
		}
	}

	/* Fullscreen safe areas */
	.class-video-container:fullscreen,
	.class-video-container:-webkit-full-screen {
		padding: env(safe-area-inset-top, 0) env(safe-area-inset-right, 0) env(safe-area-inset-bottom, 0) env(safe-area-inset-left, 0);
		border-radius: 0;
	}

	/* Reduced motion preference */
	@media (prefers-reduced-motion: reduce) {
		.class-video-container,
		.class-video-container video {
			transition: none;
		}
	}

	/* High contrast mode */
	@media (prefers-contrast: high) {
		.video-overlay {
			border-bottom-width: 2px;
		}
	}
</style>
