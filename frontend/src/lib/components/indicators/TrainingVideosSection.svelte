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
						<div id="{video.id}" class="video-player"></div>
						<video 
							id="{video.id}" 
							controls 
							width="100%" 
							poster="{video.posterUrl}"
							style="aspect-ratio: 16/9;" 
							title="{video.title}"
						>
							<source src="{video.videoUrl}" type="video/mp4">
							Your browser does not support the video tag.
						</video>
					</div>
				</div>
			{/each}
		</div>
	</div>
</section>

<style>
	.ca-section {
		padding-bottom: 0;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0;
	}

	.ca-content-block {
		margin: 0;
		padding: 0;
	}

	.current-vid {
		width: 100%;
		background-color: #f4f4f4;
		padding: 10px 10px 0;
	}

	.current-vid:last-child {
		padding-bottom: 25px;
	}

	.video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid #999;
		cursor: pointer;
	}

	.video-container.current {
		width: 100%;
		display: flex;
		z-index: 1;
	}

	.video-container video {
		width: 100%;
		display: block;
	}

	.video-overlay {
		background-color: rgba(0, 0, 0, 0.269);
	}

	@media (min-width: 416px) {
		.current-vid {
			padding: 25px 25px 0;
		}

		.current-vid:last-child {
			padding-bottom: 25px;
		}
	}
</style>
