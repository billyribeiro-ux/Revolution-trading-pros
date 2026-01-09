<svelte:head>
	<title>Quickstart To Precision Trading - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Quickstart To Precision Trading" />
	<meta property="og:site_name" content="Simpler Trading" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';
	import ClassHeaderSection from '$lib/components/classes/ClassHeaderSection.svelte';
	import ClassDownloadsSection from '$lib/components/classes/ClassDownloadsSection.svelte';

	onMount(() => {
		if (typeof window !== 'undefined' && (window as any).richpanel) {
			(window as any).richpanel.track('view_article', {
				id: 1142327,
				name: 'Quickstart To Precision Trading',
				url: window.location.href
			});
		}

		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('event', 'page_view', {
				page_title: 'Quickstart To Precision Trading',
				page_location: window.location.href,
				page_path: window.location.pathname,
				content_type: 'article'
			});
		}
	});
</script>

<div class="class-page-wrapper">
	<div class="class-page-container">
		<ClassHeaderSection title="Quickstart To Precision Trading" />

		<section class="class-section cpost-section" id="class-recordings">
			<div class="section-inner">
				<div class="class-content-block cpost-content-block">
					<div class="current-vid">
						<h2 class="class-player-header">Quickstart To Precision Trading</h2>
						<h3 class="current-title">Quickstart To Precision Trading</h3>
						<div class="class-video-container current">
							<video
								id="quickstart-video"
								controls
								width="100%"
								style="aspect-ratio: 16/9;"
								title="quickstart-to-precision-trading"
							>
								<source
									src="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4"
									type="video/mp4"
								/>
								Your browser does not support the video tag.
							</video>
						</div>
					</div>
				</div>
			</div>
		</section>

		<ClassDownloadsSection 
			boxUrl="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
		/>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Quickstart To Precision Trading - Apple ICT 7 Standards
	 * Pixel-perfect WordPress match with white background and centered layout
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-page-wrapper {
		background-color: #ffffff;
		min-height: 100vh;
		width: 100%;
	}

	.class-page-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 40px 20px;
		background-color: #ffffff;
	}

	.class-section {
		margin-bottom: 40px;
	}

	.section-inner {
		width: 100%;
	}

	/* Title Section - Now using ClassHeaderSection component (SSOT) */

	/* Video Headers - Centered */
	.class-player-header {
		font-size: 1.8rem;
		font-weight: 600;
		color: #333333;
		margin-bottom: 10px;
		text-align: center;
	}

	.current-title {
		font-size: 1.4rem;
		font-weight: 500;
		color: #666666;
		margin-bottom: 25px;
		text-align: center;
	}

	.class-video-container {
		width: 100%;
		margin-bottom: 30px;
		background: #000000;
		border-radius: 4px;
		overflow: hidden;
	}

	.class-video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	/* Downloads Section - Now using ClassDownloadsSection component (SSOT) */

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Design - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.class-page-container {
			padding: 30px 15px;
		}

		/* Header responsive styles now in ClassHeaderSection component */

		.class-player-header {
			font-size: 1.5rem;
		}

		.current-title {
			font-size: 1.2rem;
		}

		/* Downloads responsive styles now in ClassDownloadsSection component */
	}

	@media (max-width: 480px) {
		.class-page-container {
			padding: 20px 10px;
		}

		/* Header responsive styles now in ClassHeaderSection component */

		.class-player-header {
			font-size: 1.3rem;
		}

		.current-title {
			font-size: 1.1rem;
		}
	}
</style>
