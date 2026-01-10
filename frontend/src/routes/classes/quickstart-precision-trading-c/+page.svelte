<svelte:head>
	<title>Quickstart To Precision Trading - Simpler Trading</title>
	<meta property="og:locale" content="en_US" />
	<meta property="og:type" content="article" />
	<meta property="og:title" content="Quickstart To Precision Trading" />
	<meta property="og:site_name" content="Simpler Trading" />
</svelte:head>

<script lang="ts">
	import { onMount } from 'svelte';
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	import HaveQuestionsSection from '$lib/components/sections/HaveQuestionsSection.svelte';

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

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- WordPress Wrapper Structure -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<!-- section#class-info.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-info">
			<div class="section-inner">
				<div id="non-member-class-upsell">
					<div id="non-member-class-title">
						<h1>Quickstart To Precision Trading</h1>
						<!-- svelte-ignore a11y_missing_content -->
						<h3></h3>
					</div>
					<div id="non-member-class-description">
						<p></p>
					</div>
				</div>
			</div>
		</section>

		<!-- section#classroom-access.class-section.cpost-section -->
		<section class="class-section cpost-section" id="classroom-access">
			<div class="section-inner">
			</div>
		</section>

		<!-- section#class-recordings.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-recordings">
			<div class="section-inner">
				<div class="class-content-block cpost-content-block">
					<div class="current-vid">
						<h2 class="class-player-header">Quickstart To Precision Trading</h2>
						<h3 class="current-title">Quickstart To Precision Trading</h3>
						<div class="class-video-description">
							<p></p>
						</div>
						<div class="class-video-container current">
							<div class="video-overlay">
								<h3>Quickstart To Precision Trading</h3>
							</div>
							<div id="quickstart-to-precision-trading" class="class-video-player"></div>
							<video 
								id="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4" 
								controls 
								width="100%" 
								poster="" 
								style="aspect-ratio: 16/9;" 
								title="quickstart-to-precision-trading"
							>
								<source src="https://simpler-options.s3.amazonaws.com/Moxie/MoxieQuickstart_A1.mp4" type="video/mp4">
								Your browser does not support the video tag.
							</video>
						</div>
					</div>
					<div class=""></div>
				</div>
			</div>
		</section>

		<!-- section#class-elearning.class-section.cpost-section -->
		<section class="class-section cpost-section" id="class-elearning">
			<div class="section-inner">
			</div>
		</section>

		<!-- section#dl-rp-row.class-section.cpost-section -->
		<section class="class-section cpost-section" id="dl-rp-row">
			<div class="section-inner">
				<section class="class-subsection" id="class-downloads">
					<h2>Class Downloads</h2>
					<div class="class-downloads-container">
						<iframe 
							src="https://simplertrading.app.box.com/embed/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq?sortColumn=date&view=list" 
							width="500" 
							height="400" 
							allowfullscreen
							title="Class Downloads"
							loading="lazy"
							referrerpolicy="strict-origin-when-cross-origin"
							sandbox="allow-scripts allow-same-origin allow-forms allow-popups allow-popups-to-escape-sandbox"
						></iframe>
						<noscript>
							<p class="iframe-fallback">Please enable JavaScript to view class downloads, or <a href="https://simplertrading.app.box.com/s/ith1lbi9t3v91z5qnrphr8q4dz0mu6xq" target="_blank" rel="noopener noreferrer">click here to open in Box.com</a>.</p>
						</noscript>
					</div>
				</section>
			</div>
		</section>
	</div>
</div>

<!-- Have Questions Section -->
<HaveQuestionsSection 
	email="support@revolutiontradingpros.com" 
	phone="8002668659" 
	phoneDisplay="(800) 266-8659" 
/>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * WORDPRESS EXACT MATCH - from CLASS_PAGE_REFERENCE.md
	 * Source: https://my.simplertrading.com/classes/quickstart-precision-trading-c
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* 1. BODY BACKGROUND - Light gray per WordPress */
	:global(html),
	:global(body) {
		background-color: #EFEFEF !important;
		color: #666666;
		font-family: "Open Sans", sans-serif;
	}

	:global(a), :global(a:visited) {
		color: #1e73be;
	}

	:global(a:hover), :global(a:focus), :global(a:active) {
		color: #000000;
	}

	/* 2. PAGE WRAPPER */
	#page {
		max-width: 100%;
		margin: 0;
		padding: 0;
	}

	#content {
		max-width: 100%;
	}

	/* 3. SECTION STRUCTURE - Mobile First */
	.cpost-section {
		padding: 40px 0;
	}

	.cpost-section .section-inner {
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* 4. CLASS RECORDINGS SECTION */
	#class-recordings {
		padding-top: 0;
	}

	.cpost-content-block.class-content-block {
		padding: 0 40px 40px;
		border-radius: 0;
		background-color: #ffffff;
	}

	/* 5. VIDEO SECTION - Dark Blue Background */
	.current-vid {
		width: 100%;
		background-color: #0a2335;
		padding: 10px 10px 25px;
	}

	.class-player-header {
		color: #ffffff;
		font-size: 1.3rem;
		font-weight: 600;
		margin-bottom: 10px;
		text-align: center;
	}

	.current-title {
		color: #d5d5d5;
		font-size: 1.1rem;
		font-weight: 500;
		margin-bottom: 15px;
		text-align: center;
	}

	.class-video-description {
		margin-bottom: 10px;
	}

	.class-video-container {
		position: relative;
		background: #000;
		overflow: hidden;
		border: 1px solid #999;
		cursor: pointer;
	}

	.class-video-container.current {
		width: 100%;
		display: flex;
		z-index: 1;
	}

	.class-video-container video {
		width: 100%;
		display: block;
	}

	.video-overlay {
		background-color: rgba(0, 0, 0, 0.269);
	}

	.video-overlay h3 {
		display: none;
	}

	/* 6. CLASS INFO SECTION (Non-Member Upsell) */
	#class-info {
		padding-top: 60px;
		padding-bottom: 60px;
	}

	#non-member-class-upsell {
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
		text-align: center;
		padding: 20px;
		background-color: #f4f4f4;
		max-width: 1020px;
		min-height: 148px;
		margin: 0 auto;
		width: 100%;
		box-sizing: border-box;
	}

	#non-member-class-title h1 {
		font-size: 2rem;
		font-weight: 700;
		color: #666666;
		margin-bottom: 10px;
		line-height: 1.2;
	}

	#non-member-class-title h3 {
		font-size: 1.2rem;
		font-weight: 400;
		color: #999999;
		margin-bottom: 15px;
	}

	#non-member-class-description {
		color: #666666;
		font-size: 1rem;
		line-height: 1.6;
	}

	/* Empty sections - WordPress placeholder */
	#classroom-access,
	#class-elearning {
		padding: 0;
	}

	#classroom-access .section-inner,
	#class-elearning .section-inner {
		display: none;
	}

	/* 7. DOWNLOADS SECTION */
	#dl-rp-row {
		padding: 40px 0;
	}

	#dl-rp-row .section-inner {
		display: flex;
		flex-direction: column;
		align-items: center;
		max-width: 1100px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.class-subsection {
		flex: 1 1;
		background-color: #fff;
		padding: 25px;
		width: 1030px;
		max-width: 1030px;
		min-height: 458px;
		box-sizing: border-box;
	}

	#class-downloads {
		margin: 0;
		width: 100%;
		box-sizing: border-box;
	}

	#class-downloads h2 {
		font-size: 1.5rem;
		font-weight: 400;
		color: #333;
		margin: 0 0 20px 0;
		text-align: left;
	}

	.class-downloads-container {
		width: 100%;
		max-width: 550px;
		height: 350px;
		box-sizing: border-box;
		border: 1px solid #ddd;
		border-radius: 8px;
		overflow: hidden;
	}

	.class-downloads-container iframe {
		width: 100%;
		height: 100%;
		border: none;
	}

	/* ICT 11 Fix: Fallback for iframe errors */
	.iframe-fallback {
		padding: 20px;
		text-align: center;
		background-color: #f4f4f4;
		border: 1px solid #ddd;
		border-radius: 4px;
		color: #666666;
	}

	.iframe-fallback a {
		color: #1e73be;
		text-decoration: underline;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE - Mobile-First (min-width breakpoints)
	 * Base styles above are mobile, these scale UP
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Mobile Small (max 480px) */
	@media (max-width: 480px) {
		#non-member-class-upsell {
			padding: 15px;
			min-height: auto;
		}

		#non-member-class-title h1 {
			font-size: 1.5rem;
		}

		.class-subsection {
			width: 100%;
			max-width: 100%;
			padding: 15px;
			min-height: auto;
		}

		#class-downloads h2 {
			font-size: 1.3rem;
			margin-bottom: 15px;
		}

		.class-downloads-container {
			max-width: 100%;
			height: 280px;
		}
	}

	/* Tablet (416px+) */
	@media (min-width: 416px) {
		.current-vid {
			padding: 25px 25px 0;
		}

		.class-player-header {
			font-size: 1.5rem;
		}

		.current-title {
			font-size: 1.2rem;
		}
	}

	/* Tablet Large (600px+) */
	@media (min-width: 600px) {
		.cpost-content-block.class-content-block {
			padding: 0 40px 40px;
		}

		.class-subsection {
			width: 100%;
			max-width: 100%;
			padding: 20px;
		}

		.class-downloads-container {
			height: 350px;
		}
	}

	/* Desktop (768px+) */
	@media (min-width: 768px) {
		.cpost-section {
			padding: 60px 0;
		}

		.cpost-content-block.class-content-block {
			padding: 0 40px 40px;
		}

		.class-player-header {
			font-size: 1.8rem;
		}

		.current-title {
			font-size: 1.4rem;
		}

		#dl-rp-row .section-inner {
			flex-direction: row;
		}

		#class-downloads {
			margin-right: 20px;
		}

		.class-subsection {
			width: 100%;
			max-width: 900px;
			padding: 25px;
		}

		.class-downloads-container {
			height: 400px;
		}
	}

	/* Desktop Large (1140px+) */
	@media (min-width: 1140px) {
		.cpost-section .section-inner {
			padding: 0;
		}

		.class-subsection {
			width: 1030px;
			max-width: 1030px;
			min-height: 458px;
		}

		#class-downloads h2 {
			font-size: 1.5rem;
		}
	}

	/* Class Info Responsive */
	@media (min-width: 768px) {
		#non-member-class-title h1 {
			font-size: 2.5rem;
		}

		#non-member-class-title h3 {
			font-size: 1.4rem;
		}
	}
</style>
