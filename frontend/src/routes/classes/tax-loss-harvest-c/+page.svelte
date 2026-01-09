<svelte:head>
	<title>Tax Loss Harvest - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Tax Loss Harvest" />
	<meta property="og:site_name" content="Simpler Trading" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';

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

<div class="class-page-wrapper">
	<div class="class-page-container">
		<section class="class-section cpost-section" id="class-info">
			<div class="section-inner">
				<div id="non-member-class-upsell">
					<div id="non-member-class-title">
						<h1>Tax Loss Harvest</h1>
					</div>
				</div>
			</div>
		</section>

		<section class="class-section cpost-section" id="class-recordings">
			<div class="section-inner">
				<div class="class-content-block cpost-content-block">
					{#each videos as video, index}
						<div class="current-vid" class:first-video={index === 0}>
							{#if index === 0}
								<h2 class="class-player-header">{video.title}</h2>
								<h3 class="current-title">{video.subtitle}</h3>
							{/if}
							<div class="class-video-container current">
								{#if index > 0}
									<div class="video-overlay">
										<h3>{video.title}</h3>
									</div>
								{/if}
								<video controls width="100%" style="aspect-ratio: 16/9;" title={video.title}>
									<source src={video.url} type="video/mp4" />
									Your browser does not support the video tag.
								</video>
							</div>
						</div>
					{/each}
				</div>
			</div>
		</section>

		<section class="class-section cpost-section" id="dl-rp-row">
			<div class="section-inner">
				<section class="class-subsection" id="class-downloads">
					<h2>Class Downloads</h2>
					<div class="class-downloads-container">
						<iframe
							src="https://simplertrading.app.box.com/embed/s/tew1quydbn8be4o9jcif8b2ops3bpipc"
							width="500"
							height="400"
							frameborder="0"
							allowfullscreen
							title="Class Downloads"
						></iframe>
					</div>
				</section>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Tax Loss Harvest Class Page - Apple ICT 7 Standards
	 * Pixel-perfect WordPress match with proper layout and centering
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

	/* Title Section */
	#non-member-class-title h1 {
		font-size: 2.5rem;
		font-weight: 700;
		color: #333333;
		margin-bottom: 30px;
		line-height: 1.2;
		text-align: center;
	}

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

	.class-video-container {
		width: 100%;
		margin-bottom: 20px;
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
		margin: 0;
	}

	.class-video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	/* Downloads Section */
	#class-downloads {
		text-align: center;
	}

	#class-downloads h2 {
		font-size: 1.8rem;
		font-weight: 600;
		color: #333333;
		margin-bottom: 25px;
	}

	.class-downloads-container {
		width: 100%;
		max-width: 500px;
		margin: 0 auto;
	}

	.class-downloads-container iframe {
		width: 100%;
		border: 1px solid #dddddd;
		border-radius: 4px;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Responsive Design - Mobile First
	 * ═══════════════════════════════════════════════════════════════════════════ */

	@media (max-width: 768px) {
		.class-page-container {
			padding: 30px 15px;
		}

		#non-member-class-title h1 {
			font-size: 2rem;
		}

		.class-player-header {
			font-size: 1.5rem;
		}

		.current-title {
			font-size: 1.2rem;
		}

		.video-overlay h3 {
			font-size: 1.1rem;
		}

		#class-downloads h2 {
			font-size: 1.5rem;
		}

		.class-downloads-container {
			max-width: 100%;
		}

		.class-downloads-container iframe {
			height: 300px;
		}
	}

	@media (max-width: 480px) {
		.class-page-container {
			padding: 20px 10px;
		}

		#non-member-class-title h1 {
			font-size: 1.75rem;
		}

		.class-player-header {
			font-size: 1.3rem;
		}

		.current-title {
			font-size: 1.1rem;
		}
	}
</style>
