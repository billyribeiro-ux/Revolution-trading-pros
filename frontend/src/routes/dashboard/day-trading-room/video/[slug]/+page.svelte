<!--
	URL: /dashboard/day-trading-room/video/[slug]
	
	Video Detail Page - Pixel-Perfect Match to MasterDash
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Based on LatestUpdatesPage reference
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	// import { page } from '$app/stores'; // TODO: Will be used when fetching video data by slug
	import CourseSecondaryNav from '$lib/components/dashboard/CourseSecondaryNav.svelte';

	const secondaryNavItems = [
		{ href: '/dashboard/day-trading-room', icon: 'layout-dashboard', text: 'Day Trading Room Dashboard' },
		{ href: '/dashboard/day-trading-room/daily-videos', icon: 'video', text: 'Premium Daily Videos' },
		{ href: '/dashboard/day-trading-room/learning-center', icon: 'school', text: 'Learning Center' },
		{ href: '/dashboard/day-trading-room/trading-room-archive', icon: 'archive', text: 'Trading Room Archives' },
		{
			href: '#',
			icon: 'users',
			text: 'Meet the Traders',
			submenu: [
				{ href: '/dashboard/day-trading-room/traders/lead-trader', icon: '', text: 'Lead Trader' },
				{ href: '/dashboard/day-trading-room/traders/senior-analyst', icon: '', text: 'Senior Analyst' },
				{ href: '/dashboard/day-trading-room/traders/head-moderator', icon: '', text: 'Head Moderator' }
			]
		},
		{
			href: '#',
			icon: 'shopping-cart',
			text: 'Trader Store',
			submenu: [
				{ href: '/dashboard/day-trading-room/store/indicators', icon: '', text: 'Indicators' },
				{ href: '/dashboard/day-trading-room/store/courses', icon: '', text: 'Advanced Courses' },
				{ href: '/dashboard/day-trading-room/store/tools', icon: '', text: 'Trading Tools' }
			]
		}
	];

	// Mock video data - will be replaced with API call
	// TODO: Fetch video data based on slug from API
	// const slug = $page.params.slug;
	
	const video = {
		title: 'Market Analysis: SPX 0DTE Strategy',
		author: 'Lead Trader',
		date: 'January 2, 2026',
		videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec29DSG.mp4',
		posterUrl: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg',
		description: '<p>Deep dive into today\'s SPX levels and key gamma zones for optimal 0DTE entries. Learn how to identify high-probability setups and manage risk effectively in fast-moving markets.</p>',
		previousVideo: {
			title: 'Trading Psychology: Managing Emotions',
			slug: 'trading-psychology-managing-emotions'
		},
		nextVideo: {
			title: 'Advanced Order Flow Analysis',
			slug: 'advanced-order-flow-analysis'
		}
	};

	const relatedVideos = [
		{
			title: 'A Cautious Entry Into 2026',
			slug: 'cautious-entry-2026',
			author: 'HG',
			date: 'December 31, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			excerpt: 'If Santa doesn\'t show up, the first bit of 2026 may be a little precarious. With that in mind, let\'s dive in to some of the most important charts for the new year.'
		},
		{
			title: 'SPX Snoozefest',
			slug: 'spx-snoozefest',
			author: 'Heather',
			date: 'December 30, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/11/18171423/MTT_HV.jpg',
			excerpt: 'We\'ve had two days of some very narrow ranges in the indices. It\'s almost as though the market has had an amazing year and just needs to rest a bit before making its next move!'
		},
		{
			title: 'Signal & Noise',
			slug: 'signal-noise',
			author: 'Sam',
			date: 'December 26, 2025',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
			excerpt: 'The signal we have been tracking in the Nasdaq continues to build. As we approach the new year, where are the pockets of strength?'
		}
	];
</script>

<svelte:head>
	<title>{video.title} | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content={video.description.replace(/<[^>]*>/g, '')} />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<CourseSecondaryNav courseName="Day Trading Room" menuItems={secondaryNavItems} />

<!-- Video Title Section -->
<section id="dv-title" class="dv-section cpost-title-section cpost-section">
	<div class="section-inner">
		{#if video.previousVideo}
			<div id="dv-previous" class="cpost-previous">
				<a href="/dashboard/day-trading-room/video/{video.previousVideo.slug}" title={video.previousVideo.title}>
					<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
				</a>
			</div>
		{/if}
		
		<h1 class="cpost-title">{video.title}</h1>
		
		{#if video.nextVideo}
			<div id="dv-next" class="cpost-next">
				<a href="/dashboard/day-trading-room/video/{video.nextVideo.slug}" title={video.nextVideo.title}>
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
					<video id="dv-player" controls width="100%" poster={video.posterUrl} style="aspect-ratio: 16/9;">
						<source src={video.videoUrl} type="video/mp4">
						Your browser does not support the video tag.
					</video>
				</div>
			</div>
		</div>
		<div class="dv-description">
			{@html video.description}
		</div>
	</div>
</section>

<!-- Related Videos Section -->
<section id="dv-recent" class="dv-section cpost-recent-section cpost-section">
	<div class="section-inner">
		<h2>Recent Day Trading Room Daily Videos</h2>
		
		<div class="card-grid flex-grid row">
			{#each relatedVideos as relatedVideo}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a href="/dashboard/day-trading-room/video/{relatedVideo.slug}" class="card-image" style="background-image: url({relatedVideo.thumbnail});">
								<img src="https://placehold.it/325x183" alt={relatedVideo.title}>
							</a>
						</figure>
						<section class="card-body">
							<h4 class="h5 card-title">
								<a href="/dashboard/day-trading-room/video/{relatedVideo.slug}">
									{relatedVideo.title}
								</a>
							</h4>
							<span class="article-card__meta"><small>{relatedVideo.date} with {relatedVideo.author}</small></span><br>
							<div class="card-description">
								<div class="u--hide-read-more u--squash">{relatedVideo.excerpt}</div>
							</div>
						</section>
						<footer class="card-footer">
							<a class="btn btn-tiny btn-default" href="/dashboard/day-trading-room/video/{relatedVideo.slug}">Watch Now</a>
						</footer>
					</div>
				</article>
			{/each}
		</div>
	</div>
</section>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * VIDEO DETAIL PAGE STYLES - Pixel-Perfect Match to MasterDash
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Title Section */
	.dv-section {
		padding: 30px 0;
	}

	.cpost-title-section {
		background: #f4f4f4;
		border-bottom: 1px solid #e5e5e5;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
		position: relative;
	}

	.cpost-title {
		font-size: 2.5rem;
		font-weight: 700;
		color: #191717;
		text-align: center;
		margin: 0 0 10px;
		font-family: 'Open Sans', sans-serif;
	}

	.cpost-subtitle {
		font-size: 1.3rem;
		font-weight: 400;
		color: #666;
		text-align: center;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
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
		color: #143E59;
		text-decoration: none;
		font-size: 1rem;
		font-weight: 600;
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #076a8a;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 1.5rem;
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

	.video-container video {
		width: 100%;
		height: auto;
		display: block;
		background: #000;
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
		color: #143E59;
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

	/* Responsive */
	@media (max-width: 768px) {
		.cpost-title {
			font-size: 1.8rem;
		}

		.cpost-subtitle {
			font-size: 1.1rem;
		}

		.cpost-previous,
		.cpost-next {
			position: static;
			transform: none;
			text-align: center;
			margin: 10px 0;
		}

		.cpost-previous {
			order: -1;
		}

		.cpost-next {
			order: 1;
		}

		.section-inner {
			display: flex;
			flex-direction: column;
		}
	}
</style>
