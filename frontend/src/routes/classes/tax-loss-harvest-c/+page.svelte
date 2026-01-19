<script lang="ts">
	import { onMount } from 'svelte';
	import ClassHeaderSection from '$lib/components/classes/ClassHeaderSection.svelte';
	import ClassVideoSection from '$lib/components/classes/ClassVideoSection.svelte';
	import ClassDownloadsSection from '$lib/components/classes/ClassDownloadsSection.svelte';

	const videos = [
		{
			title: 'Tax Loss Harvest 2025',
			subtitle: 'Wed, Dec 31, 2025, PM - JC',
			url: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_JC_12-31-2025__03.00.276_PM.mp4'
		},
		{
			title: 'Wed, Dec 31, 2025, PM - Henry',
			subtitle: '',
			url: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_HG_12-31-2025__02.00.196_PM.mp4'
		},
		{
			title: 'Wed, Dec 31, 2025, PM - Taylor',
			subtitle: '',
			url: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_Taylor_Horton_12-31-2025__12.30.925_PM.mp4'
		},
		{
			title: 'Wed, Dec 31, 2025, AM - JC',
			subtitle: '',
			url: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_JC_12-31-2025__09.23.835_AM.mp4'
		},
		{
			title: 'Thu, Dec 18, 2025',
			subtitle: '',
			url: 'https://cloud-streaming.s3.amazonaws.com/chatrecordings/STWebinars/6941b999d4e2e95d75e7ddfb_Tax_Loss_Harvest_2025_JC_12-18-2025__04.02.148_PM.mp4'
		}
	];

	onMount(() => {
		if (typeof window !== 'undefined' && (window as any).richpanel) {
			(window as any).richpanel.track('view_article', {
				id: 2173470,
				name: 'Tax Loss Harvest',
				url: window.location.href
			});
		}

		if (typeof window !== 'undefined' && (window as any).gtag) {
			(window as any).gtag('event', 'page_view', {
				page_title: 'Tax Loss Harvest',
				page_location: window.location.href,
				page_path: window.location.pathname,
				content_type: 'article'
			});
		}
	});
</script>

<svelte:head>
	<title>Tax Loss Harvest - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Tax Loss Harvest" />
	<meta property="og:site_name" content="Simpler Trading" />
</svelte:head>

<div class="class-page-wrapper">
	<div class="class-page-container">
		<ClassHeaderSection title="Tax Loss Harvest" />

		<section class="class-section cpost-section" id="class-recordings">
			<div class="section-inner">
				<div class="class-content-block cpost-content-block">
					{#each videos as video, index}
						<div class="current-vid" class:first-video={index === 0}>
							{#if index === 0}
								<h2 class="class-player-header">{video.title}</h2>
								<h3 class="current-title">{video.subtitle}</h3>
							{/if}
							<ClassVideoSection
								videoUrl={video.url}
								videoTitle={video.title}
								overlayTitle={index > 0 ? video.title : ''}
								showOverlay={index > 0}
							/>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<ClassDownloadsSection courseSlug="tax-loss-harvest-c" />
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Tax Loss Harvest Class Page - Apple ICT 7 Standards
	 * Pixel-perfect WordPress match with proper layout and centering
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.class-page-wrapper {
		background-color: #ffffff;
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

	/* Video Section */
	.class-content-block {
		width: 100%;
	}

	.current-vid {
		margin-bottom: 30px;
	}

	.current-vid.first-video {
		margin-bottom: 40px;
	}

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

	/* Video container - Now using ClassVideoSection component (SSOT) */

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

		/* Video responsive styles now in ClassVideoSection component */

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
