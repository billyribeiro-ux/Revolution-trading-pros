<!--
	URL: /dashboard/swing-trading-room/video/[slug]

	Video Detail Page - Pixel-Perfect Match to MasterDash (Swing Trading Room)
	===============================================================================
	Apple ICT 11+ Principal Engineer Implementation
	Based on LatestUpdatesPage reference

	ICT 7 FIX: Now uses BunnyVideoPlayer with progress tracking

	@version 2.0.0 - January 2026 - Progress Tracking Enabled
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { sanitizeHtml } from '$lib/utils/sanitize';
	import type { PageData } from './+page.server';
	import BunnyVideoPlayer from '$lib/components/video/BunnyVideoPlayer.svelte';
	import { authStore } from '$lib/stores/auth.svelte';

	// Server-loaded data with Previous/Next navigation
	let props: { data: PageData } = $props();
	let data = $derived(props.data);

	// Reactive video data from server
	const video = $derived(data.video);

	// Get user ID for progress tracking
	const userId = $derived(authStore.user?.id);

	// Extract Bunny.net video details from URL
	const extractBunnyDetails = (url: string) => {
		const match = url.match(/\/embed\/(\d+)\/([a-f0-9-]+)/);
		if (match) {
			return { libraryId: match[1], videoGuid: match[2] };
		}
		return { libraryId: '', videoGuid: '' };
	};

	const bunnyDetails = $derived(extractBunnyDetails(video.videoUrl));

	// Related videos (could also be fetched from server in the future)
	const relatedVideos = [
		{
			title: 'Swing Trading Basics',
			slug: 'swing-trading-basics',
			author: 'Trading Team',
			date: 'December 31, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			excerpt:
				'Learn the core concepts of swing trading and how to identify profitable opportunities in any market condition.'
		},
		{
			title: 'Market Structure Analysis',
			slug: 'market-structure-analysis',
			author: 'Trading Team',
			date: 'December 30, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
			excerpt:
				'Deep dive into market structure and how to use it to make better trading decisions for swing trades.'
		},
		{
			title: 'Weekly Trading Plan',
			slug: 'weekly-trading-plan',
			author: 'Trading Team',
			date: 'December 26, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			excerpt:
				'How to set up your weekly trading plan for consistent swing trading success and better risk management.'
		}
	];
</script>

<svelte:head>
	<title>{video.title} | Swing Trading Room | Revolution Trading Pros</title>
	<meta name="description" content={video.description.replace(/<[^>]*>/g, '')} />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Video Title Section -->
<section id="dv-title" class="dv-section cpost-title-section cpost-section">
	<div class="section-inner">
		{#if video.previousVideo}
			<div id="dv-previous" class="cpost-previous">
				<a
					href="/dashboard/swing-trading-room/video/{video.previousVideo.slug}"
					title={video.previousVideo.title}
				>
					<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
				</a>
			</div>
		{/if}

		<h1 class="cpost-title">{video.title}</h1>

		{#if video.nextVideo}
			<div id="dv-next" class="cpost-next">
				<a
					href="/dashboard/swing-trading-room/video/{video.nextVideo.slug}"
					title={video.nextVideo.title}
				>
					<span>Next </span><i class="fa fa-chevron-circle-right"></i>
				</a>
			</div>
		{/if}

		<h2 class="cpost-subtitle">With {video.author}</h2>
	</div>
</section>

<!-- Video Player Section -->
<section id="dv-main" class="dv-section cpost-section">
	<div class="section-inner">
		<div class="dv-content-block cpost-content-block w-desc">
			<div class="current-vid">
				<div class="video-container current">
					<!-- ICT 7 FIX: Use BunnyVideoPlayer with progress tracking -->
					<BunnyVideoPlayer
						videoId={typeof video.id === 'string' ? parseInt(video.id, 10) : video.id}
						videoGuid={bunnyDetails.videoGuid}
						libraryId={bunnyDetails.libraryId}
						thumbnailUrl={video.thumbnailUrl}
						title={video.title}
						{userId}
						autoplay={false}
						muted={false}
					/>
				</div>
			</div>
		</div>
		<div class="dv-description">
			{@html sanitizeHtml(video.description, 'standard')}
		</div>
	</div>
</section>

<!-- Related Videos Section -->
<section id="dv-recent" class="dv-section cpost-recent-section cpost-section">
	<div class="section-inner">
		<h2>Recent Swing Trading Room Videos</h2>

		<div class="card-grid flex-grid row">
			{#each relatedVideos as relatedVideo}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a
								href="/dashboard/swing-trading-room/video/{relatedVideo.slug}"
								class="card-image"
								style="background-image: url({relatedVideo.thumbnail});"
							>
								<img src="https://placehold.it/325x183" alt={relatedVideo.title} />
							</a>
						</figure>
						<section class="card-body">
							<h4 class="h5 card-title">
								<a href="/dashboard/swing-trading-room/video/{relatedVideo.slug}">
									{relatedVideo.title}
								</a>
							</h4>
							<span class="article-card__meta"
								><small>{relatedVideo.date} with {relatedVideo.author}</small></span
							><br />
							<div class="card-description">
								<div class="u--hide-read-more u--squash">{relatedVideo.excerpt}</div>
							</div>
						</section>
						<footer class="card-footer">
							<a
								class="btn btn-tiny btn-default"
								href="/dashboard/swing-trading-room/video/{relatedVideo.slug}">Watch Now</a
							>
						</footer>
					</div>
				</article>
			{/each}
		</div>
	</div>
</section>

<style>
	/* ===============================================================================
	 * VIDEO DETAIL PAGE - 2026 Mobile-First Responsive Design
	 * ===============================================================================
	 * Breakpoints: xs(360px), sm(640px), md(768px), lg(1024px), xl(1280px)
	 * Touch Targets: 44x44px minimum
	 * Safe Areas: env(safe-area-inset-*) for notched devices
	 * =============================================================================== */

	/* Mobile-First Base - Title Section */
	.dv-section {
		padding: 20px 0;
	}

	.cpost-title-section {
		background: #f4f4f4;
		border-bottom: 1px solid #e5e5e5;
		padding-top: max(20px, env(safe-area-inset-top, 20px));
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 16px;
		padding-left: max(16px, env(safe-area-inset-left, 16px));
		padding-right: max(16px, env(safe-area-inset-right, 16px));
		position: relative;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 16px;
	}

	/* Mobile-First Title - Smaller on mobile */
	.cpost-title {
		font-size: 1.5rem;
		font-weight: 700;
		color: #191717;
		text-align: center;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.3;
		order: 1;
	}

	.cpost-subtitle {
		font-size: 1rem;
		font-weight: 400;
		color: #666;
		text-align: center;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
		order: 2;
	}

	/* Mobile Navigation - Stacked above title */
	.cpost-previous,
	.cpost-next {
		position: static;
		transform: none;
	}

	/* Navigation links - Touch-friendly (44px target) */
	.cpost-previous a,
	.cpost-next a {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		min-width: 44px;
		padding: 10px 16px;
		color: #143e59;
		text-decoration: none;
		font-size: 0.875rem;
		font-weight: 600;
		gap: 8px;
		border-radius: 8px;
		background: rgba(20, 62, 89, 0.05);
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
		transition: all 0.2s ease;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #076a8a;
		background: rgba(20, 62, 89, 0.1);
	}

	.cpost-previous a:focus-visible,
	.cpost-next a:focus-visible {
		outline: 2px solid #143e59;
		outline-offset: 2px;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 1.25rem;
	}

	/* Video Player Section */
	.cpost-section {
		background: #fff;
	}

	.dv-content-block {
		margin-bottom: 30px;
	}

	.video-container {
		position: relative;
		width: 100%;
		max-width: 1000px;
		margin: 0 auto;
	}

	/* BunnyVideoPlayer fills container - no direct video styles needed */
	.video-container :global(.bunny-video-player) {
		width: 100%;
		border-radius: 8px;
		overflow: hidden;
	}

	.dv-description {
		max-width: 1000px;
		margin: 0 auto;
		font-size: 1.1rem;
		line-height: 1.8;
		color: #555;
	}

	.dv-description :global(p) {
		margin-bottom: 15px;
	}

	/* Related Videos Section */
	.cpost-recent-section {
		background: #f8f9fa;
		padding: 50px 0;
	}

	.cpost-recent-section h2 {
		font-size: 2rem;
		font-weight: 700;
		color: #191717;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
	}

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -10px;
	}

	.card-grid-spacer {
		padding: 0 10px 20px;
	}

	.card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		overflow: hidden;
	}

	.card-image {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		display: block;
	}

	.card-image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(9, 132, 174, 0.9);
		border-radius: 50%;
		pointer-events: none;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.card-title {
		font-size: 1.2rem;
		font-weight: 700;
		margin-bottom: 10px;
	}

	.card-title a {
		color: #191717;
		text-decoration: none;
	}

	.card-title a:hover {
		color: #143e59;
	}

	.card-description {
		margin-top: 10px;
		color: #666;
		line-height: 1.6;
	}

	.u--hide-read-more {
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-footer {
		padding: 0 20px 20px;
	}

	/* ===============================================================================
	 * RESPONSIVE BREAKPOINTS - Progressive Enhancement
	 * =============================================================================== */

	/* xs: 360px+ - Small phones */
	@media (min-width: 360px) {
		.section-inner {
			padding: 0 20px;
		}

		.cpost-title {
			font-size: 1.625rem;
		}
	}

	/* sm: 640px+ - Large phones / small tablets */
	@media (min-width: 640px) {
		.dv-section {
			padding: 24px 0;
		}

		.section-inner {
			flex-direction: row;
			flex-wrap: wrap;
			justify-content: center;
			gap: 12px;
		}

		.cpost-title {
			font-size: 1.875rem;
			width: 100%;
			order: 0;
		}

		.cpost-subtitle {
			font-size: 1.1rem;
			width: 100%;
			order: 1;
		}

		.cpost-previous {
			order: 2;
		}

		.cpost-next {
			order: 3;
		}
	}

	/* md: 768px+ - Tablets */
	@media (min-width: 768px) {
		.dv-section {
			padding: 28px 0;
		}

		.section-inner {
			position: relative;
			display: block;
			padding: 0 80px;
		}

		.cpost-title {
			font-size: 2.25rem;
		}

		.cpost-previous,
		.cpost-next {
			position: absolute;
			top: 50%;
			transform: translateY(-50%);
		}

		.cpost-previous {
			left: 20px;
		}

		.cpost-next {
			right: 20px;
		}

		.cpost-previous a,
		.cpost-next a {
			font-size: 1rem;
		}

		.cpost-previous i,
		.cpost-next i {
			font-size: 1.5rem;
		}
	}

	/* lg: 1024px+ - Desktop */
	@media (min-width: 1024px) {
		.dv-section {
			padding: 30px 0;
		}

		.cpost-title {
			font-size: 2.5rem;
		}

		.cpost-subtitle {
			font-size: 1.3rem;
		}
	}

	/* High contrast / reduced motion preferences */
	@media (prefers-reduced-motion: reduce) {
		.cpost-previous a,
		.cpost-next a,
		.btn,
		.card {
			transition: none;
		}

		.card:hover {
			transform: none;
		}
	}

	/* Landscape on mobile - optimize video viewing */
	@media (max-width: 767px) and (orientation: landscape) {
		.dv-section {
			padding: 12px 0;
		}

		.cpost-title {
			font-size: 1.25rem;
		}

		.cpost-subtitle {
			font-size: 0.875rem;
		}

		/* BunnyVideoPlayer handles its own height constraints */
		.video-container {
			max-height: 70dvh;
		}
	}

	/* Grid columns */
	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			width: 33.333333%;
		}
	}

	/* Button styles */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-height: 44px;
		min-width: 44px;
		padding: 12px 20px;
		border-radius: 8px;
		text-decoration: none;
		font-weight: 600;
		font-size: 0.8125rem;
		text-transform: uppercase;
		letter-spacing: 0.5px;
		transition: all 0.2s ease;
		cursor: pointer;
		-webkit-tap-highlight-color: transparent;
		touch-action: manipulation;
	}

	.btn-tiny {
		padding: 10px 16px;
		font-size: 0.75rem;
		min-height: 44px;
	}

	.btn-default {
		background: #f7941d;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #e5850f;
		color: #fff;
	}

	.btn-default:focus-visible {
		outline: 2px solid #f7941d;
		outline-offset: 2px;
	}
</style>
