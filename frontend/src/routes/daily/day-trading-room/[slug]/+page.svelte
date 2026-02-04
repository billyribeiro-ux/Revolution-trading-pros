<!--
	URL: /daily/day-trading-room/[slug]
	
	Premium Daily Video Detail Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Individual video detail page matching WordPress implementation exactly.
	Includes video player, navigation, description, and related videos.
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { page } from '$app/state';
	import { browser } from '$app/environment';

	interface DailyVideo {
		id: number;
		title: string;
		date: string;
		trader: string;
		excerpt: string;
		description: string;
		slug: string;
		thumbnail: string;
		videoUrl: string;
		isVideo: boolean;
	}

	// Get slug from URL
	let slug = $derived(page.params.slug);

	// Video data state - Svelte 5 $state() runes
	let currentVideo = $state<DailyVideo | null>(null);
	let relatedVideos = $state<DailyVideo[]>([]);
	let previousVideo = $state<DailyVideo | null>(null);
	let nextVideo = $state<DailyVideo | null>(null);
	let loading = $state(true);
	let videoElement = $state<HTMLVideoElement | undefined>(undefined);

	// Video ended redirect handler
	function handleVideoEnded() {
		window.location.href = 'https://lp.simplertrading.com/shortcut';
	}

	// Sample data - matches WordPress structure
	const allVideos: DailyVideo[] = [
		{
			id: 1,
			title: 'How to use Bookmap to make more informed trades',
			date: 'January 02, 2026',
			trader: 'Kody Ashmore',
			excerpt:
				"You asked for it, you got it. Here are Kody's Bookmap tools and how he uses them to make better informed trades.",
			description:
				"You asked for it, you got it. Here are Kody's Bookmap tools and how he uses them to make better informed trades.",
			slug: 'bookmap',
			thumbnail: 'https://cdn.simplertrading.com/2025/02/07135413/SimplerCentral_KA.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/sample.mp4',
			isVideo: true
		},
		{
			id: 2,
			title: 'Ho Ho Whoa!',
			date: 'December 18, 2025',
			trader: 'Bruce Marshall',
			excerpt:
				"In this video, Bruce discusses today's market action and the outlook for the end of the year.",
			description:
				"In this video, Bruce discusses today's market action and the outlook for the end of the year. After CPI this morning, we got a nice bounce and relief rally after a long and messy chop phase since Thanksgiving. We have Quad Witching tomorrow after today's monthly (AM) expiration in the indices and then PCE next Tuesday, so lots more volatility to come. We are clinging onto the 50 SMA on the ES, and if this can hold, we still have a shot at seeing our Santa Rally this year. Bruce will continue to be a cautious Bull until we get more clarity, but don't load the sleigh too heavy just yet, as we don't have clear skies to fly in as of now.",
			slug: 'ho-ho-whoa',
			thumbnail: 'https://cdn.simplertrading.com/2025/04/07135027/SimplerCentral_BM.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/dec18BM.mp4',
			isVideo: true
		},
		{
			id: 3,
			title: 'A Cautious Entry Into 2026',
			date: 'December 31, 2025',
			trader: 'Henry Gambell',
			excerpt: "If Santa doesn't show up, the first bit of 2026 may be a little precarious.",
			description:
				"If Santa doesn't show up, the first bit of 2026 may be a little precarious. With that in mind, let's dive in to some of the most important charts for the new year.",
			slug: 'cautious-entry-into-2026',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			videoUrl: 'https://simpler-options.s3.amazonaws.com/nightlyvids/2025/sample2.mp4',
			isVideo: true
		}
	];

	// Svelte 5: $effect replaces onMount for browser-only initialization
	$effect(() => {
		if (!browser) return;

		// Find current video
		const videoIndex = allVideos.findIndex((v) => v.slug === slug);

		if (videoIndex !== -1) {
			currentVideo = allVideos[videoIndex];

			// Get previous and next videos
			if (videoIndex > 0) {
				previousVideo = allVideos[videoIndex - 1];
			}
			if (videoIndex < allVideos.length - 1) {
				nextVideo = allVideos[videoIndex + 1];
			}

			// Get related videos (exclude current)
			relatedVideos = allVideos.filter((v) => v.slug !== slug).slice(0, 3);
		}

		loading = false;
	});
</script>

<svelte:head>
	{#if currentVideo}
		<title>{currentVideo.title} | Day Trading Room | Revolution Trading Pros</title>
		<meta name="description" content={currentVideo.excerpt} />
		<meta name="robots" content="index, follow" />
	{:else}
		<title>Video Not Found | Revolution Trading Pros</title>
	{/if}
</svelte:head>

{#if loading}
	<div class="loading-container">
		<div class="loading">
			<span class="loading-icon"></span>
			<span class="loading-message">Loading video...</span>
		</div>
	</div>
{:else if !currentVideo}
	<div class="error-container">
		<h1>Video Not Found</h1>
		<p>The video you're looking for doesn't exist.</p>
		<a href="/dashboard/day-trading-room/daily-videos" class="btn btn-default">Back to Videos</a>
	</div>
{:else}
	<!-- Breadcrumbs Navigation -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home">
					<a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a>
				</li>
				<li class="separator separator-home">/</li>
				<li class="item-cat item-custom-post-type-daily">
					<a
						class="breadcrumb-cat breadcrumb-custom-post-type-daily"
						href="/dashboard/day-trading-room/daily-videos"
						title="Daily Videos">Daily Videos</a
					>
				</li>
				<li class="separator">/</li>
				<li class="item-cat"></li>
				<li class="separator">/</li>
				<li class="item-current item-{currentVideo.id}">
					<strong class="breadcrumb-current breadcrumb-{currentVideo.id}" title={currentVideo.title}
						>{currentVideo.title}</strong
					>
				</li>
			</ul>
		</div>
	</nav>

	<!-- Title Section with Previous/Next Navigation -->
	<section id="dv-title" class="dv-section cpost-title-section cpost-section">
		<div class="section-inner">
			<div id="dv-previous" class="cpost-previous">
				{#if previousVideo}
					<a href="/daily/day-trading-room/{previousVideo.slug}" title={previousVideo.title}>
						<i class="fa fa-chevron-circle-left"></i><span> Previous</span>
					</a>
				{/if}
			</div>
			<h1 class="cpost-title">{currentVideo.title}</h1>
			<div id="dv-next" class="cpost-next">
				{#if nextVideo}
					<a href="/daily/day-trading-room/{nextVideo.slug}" title={nextVideo.title}>
						<span>Next </span><i class="fa fa-chevron-circle-right"></i>
					</a>
				{/if}
			</div>
			<h2 class="cpost-subtitle">With {currentVideo.trader}</h2>
		</div>
	</section>

	<!-- Main Video Section -->
	<section id="dv-main" class="dv-section cpost-section">
		<div class="section-inner">
			<div class="dv-content-block cpost-content-block w-desc">
				<div class="current-vid">
					<div class="video-container current">
						<video
							id="dv-player"
							bind:this={videoElement}
							controls
							width="100%"
							poster={currentVideo.thumbnail}
							style="aspect-ratio: 16/9;"
							onended={handleVideoEnded}
						>
							<source src={currentVideo.videoUrl} type="video/mp4" />
							Your browser does not support the video tag.
						</video>
					</div>
				</div>
			</div>
			<div class="dv-description">
				<p>{currentVideo.description}</p>
			</div>
		</div>
	</section>

	<!-- Recent Videos Section -->
	<section id="dv-recent" class="dv-section cpost-recent-section cpost-section">
		<div class="section-inner">
			<h2>Recent Day Trading Room Daily Videos</h2>

			<div class="card-grid flex-grid row">
				{#each relatedVideos as video (video.id)}
					<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
						<div class="card flex-grid-panel">
							<figure class="card-media card-media--video">
								<a
									href="/daily/day-trading-room/{video.slug}"
									class="card-image"
									style="background-image: url({video.thumbnail});"
								>
									<img src="https://placehold.it/325x183" alt={video.title} />
								</a>
							</figure>
							<section class="card-body">
								<h4 class="h5 card-title">
									<a href="/daily/day-trading-room/{video.slug}">
										{video.title}
									</a>
								</h4>
								<span class="article-card__meta">
									<small>{video.date} with {video.trader}</small>
								</span>
								<br />
								<div class="card-description">
									<div class="u--hide-read-more u--squash">{video.excerpt}</div>
								</div>
							</section>
							<footer class="card-footer">
								<a class="btn btn-tiny btn-default" href="/daily/day-trading-room/{video.slug}">
									Watch Now
								</a>
							</footer>
						</div>
					</article>
				{/each}
			</div>
		</div>
	</section>
{/if}

<style>
	/* Breadcrumbs */
	.breadcrumbs {
		z-index: 1;
		background: #f5f5f5;
		border-bottom: 1px solid #e6e6e6;
		padding: 15px 0;
	}

	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 5px;
	}

	.breadcrumbs li {
		display: inline;
		font-size: 14px;
	}

	.breadcrumb-link,
	.breadcrumb-cat {
		color: #666;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.breadcrumb-link:hover,
	.breadcrumb-cat:hover {
		color: #f69532;
	}

	.separator {
		color: #999;
		margin: 0 5px;
	}

	.breadcrumb-current {
		color: #333;
		font-weight: 600;
	}

	/* Loading State */
	.loading-container {
		display: flex;
		justify-content: center;
		align-items: center;
		min-height: 400px;
		padding: 40px;
	}

	.loading {
		text-align: center;
	}

	.loading-icon {
		display: inline-block;
		width: 40px;
		height: 40px;
		border: 4px solid #f3f3f3;
		border-top: 4px solid #f69532;
		border-radius: 50%;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}

	.loading-message {
		display: block;
		margin-top: 15px;
		color: #666;
		font-size: 14px;
	}

	/* Error State */
	.error-container {
		text-align: center;
		padding: 60px 20px;
	}

	.error-container h1 {
		font-size: 32px;
		margin-bottom: 15px;
		color: #333;
	}

	.error-container p {
		font-size: 16px;
		color: #666;
		margin-bottom: 25px;
	}

	/* Title Section */
	.dv-section {
		padding: 30px 0;
	}

	.cpost-title-section {
		background: #f5f5f5;
		border-bottom: 1px solid #e6e6e6;
	}

	.section-inner {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
		position: relative;
	}

	.cpost-title {
		font-size: 36px;
		font-weight: 700;
		margin: 0 0 10px;
		text-align: center;
		color: #333;
		padding: 0 120px;
	}

	.cpost-subtitle {
		font-size: 18px;
		font-weight: 400;
		margin: 0;
		text-align: center;
		color: #666;
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
		display: flex;
		align-items: center;
		gap: 8px;
		color: #f69532;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
		transition: color 0.2s ease;
	}

	.cpost-previous a:hover,
	.cpost-next a:hover {
		color: #dc7309;
	}

	.cpost-previous i,
	.cpost-next i {
		font-size: 24px;
	}

	/* Main Video Section */
	.cpost-section {
		background: #fff;
	}

	.dv-content-block {
		margin-bottom: 30px;
	}

	.current-vid {
		width: 100%;
	}

	.video-container {
		position: relative;
		width: 100%;
		background: #000;
		border-radius: 4px;
		overflow: hidden;
	}

	.video-container video {
		display: block;
		width: 100%;
		height: auto;
	}

	.dv-description {
		padding: 20px 0;
		color: #333;
		font-size: 16px;
		line-height: 1.6;
	}

	.dv-description p {
		margin: 0;
	}

	/* Recent Videos Section */
	.cpost-recent-section {
		background: #f9f9f9;
		border-top: 1px solid #e6e6e6;
	}

	.cpost-recent-section h2 {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 30px;
		color: #333;
		text-align: center;
	}

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px 30px;
	}

	.flex-grid-item {
		display: flex;
	}

	.card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		overflow: hidden;
		margin: 0;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 60px;
		height: 60px;
		background: rgba(0, 0, 0, 0.7);
		border-radius: 50%;
		pointer-events: none;
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
		opacity: 0;
		width: 100%;
		height: 100%;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.card-title {
		margin: 0 0 10px;
		font-size: 18px;
		font-weight: 700;
		line-height: 1.3;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card-title a:hover {
		color: #f69532;
	}

	.article-card__meta {
		display: block;
		margin-bottom: 10px;
	}

	.article-card__meta small {
		font-size: 12px;
		color: #999;
	}

	.card-description {
		margin-top: 10px;
		font-size: 14px;
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
		padding: 15px 20px;
		border-top: 1px solid #e6e6e6;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #f69532;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Responsive Grid */
	@media (max-width: 991px) {
		.col-md-6 {
			width: 50%;
		}

		.cpost-title {
			font-size: 28px;
			padding: 0 100px;
		}
	}

	@media (max-width: 767px) {
		.col-sm-6 {
			width: 50%;
		}

		.cpost-title {
			font-size: 24px;
			padding: 0 80px;
		}

		.card-title {
			font-size: 16px;
		}

		.cpost-previous span,
		.cpost-next span {
			display: none;
		}
	}

	@media (max-width: 575px) {
		.col-xs-12 {
			width: 100%;
		}

		.card-grid {
			margin: 0 -10px;
		}

		.card-grid-spacer {
			padding: 0 10px 20px;
		}

		.cpost-title {
			font-size: 20px;
			padding: 0 60px;
		}

		.cpost-previous,
		.cpost-next {
			left: 10px;
		}

		.cpost-next {
			right: 10px;
			left: auto;
		}
	}

	@media (min-width: 1200px) {
		.col-lg-4 {
			width: 33.333%;
		}
	}
</style>
