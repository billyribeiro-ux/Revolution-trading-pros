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
		poster={poster}
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
	 * Responsive Design - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.class-video-container {
			margin-bottom: 20px;
		}

		.video-overlay h3 {
			font-size: 1.1rem;
		}

		.class-video-description p {
			font-size: 0.95rem;
		}
	}

	@media (max-width: 480px) {
		.video-overlay {
			padding: 12px 15px;
		}

		.video-overlay h3 {
			font-size: 1rem;
		}

		.class-video-description p {
			font-size: 0.9rem;
		}
	}
</style>
