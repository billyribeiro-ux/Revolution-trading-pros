<svelte:head>
	<title>Quickstart To Precision Trading - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Quickstart To Precision Trading" />
	<meta property="og:site_name" content="Simpler Trading" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';

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

<div class="class-page-container">
	<section class="class-section cpost-section" id="class-info">
		<div class="section-inner">
			<div id="non-member-class-upsell">
				<div id="non-member-class-title">
					<h1>Quickstart To Precision Trading</h1>
				</div>
			</div>
		</div>
	</section>

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

	<section class="class-section cpost-section" id="dl-rp-row">
		<div class="section-inner">
			<section class="class-subsection" id="class-downloads">
				<h2>Class Downloads</h2>
				<div class="class-downloads-container">
					<iframe
						src="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list"
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

<style>
	.class-page-container {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.class-section {
		margin-bottom: 40px;
	}

	.section-inner {
		width: 100%;
	}

	#non-member-class-title h1 {
		font-size: 2.5rem;
		font-weight: 700;
		color: #333;
		margin-bottom: 20px;
		line-height: 1.2;
	}

	.class-player-header {
		font-size: 1.8rem;
		font-weight: 600;
		color: #333;
		margin-bottom: 10px;
	}

	.current-title {
		font-size: 1.4rem;
		font-weight: 500;
		color: #666;
		margin-bottom: 20px;
	}

	.class-video-container {
		width: 100%;
		margin-bottom: 30px;
		background: #000;
		border-radius: 4px;
		overflow: hidden;
	}

	.class-video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	#class-downloads h2 {
		font-size: 1.8rem;
		font-weight: 600;
		color: #333;
		margin-bottom: 20px;
	}

	.class-downloads-container {
		width: 100%;
		max-width: 500px;
	}

	.class-downloads-container iframe {
		width: 100%;
		border: 1px solid #ddd;
		border-radius: 4px;
	}

	@media (max-width: 768px) {
		#non-member-class-title h1 {
			font-size: 2rem;
		}

		.class-player-header {
			font-size: 1.5rem;
		}

		.current-title {
			font-size: 1.2rem;
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
</style>
