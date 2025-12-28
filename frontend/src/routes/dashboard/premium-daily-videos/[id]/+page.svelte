<script lang="ts">
	import { page } from '$app/stores';

	/**
	 * Individual Premium Daily Video Page
	 * Reference: frontend/Do's/Premium-Daily-Videos-Clicked
	 */
	const videoId = $derived($page.params.id!);

	// Mock video data - in real app this would come from API
	const video = $derived({
		id: videoId,
		title: 'Signal & Noise',
		date: 'December 26, 2025',
		trader: 'Sam',
		traderImage: 'https://cdn.simplertrading.com/2025/05/07134553/SimplerCentral_SS.jpg',
		description:
			'The signal we have been tracking in the Nasdaq continues to build. As we approach the new year, where are the pockets of strength?',
		videoUrl: 'https://cdn.jwplayer.com/videos/sample.mp4'
	});

	const relatedVideos = [
		{
			id: 'a-strong-end-of-year',
			title: 'A Strong End Of Year',
			date: 'December 24, 2025',
			trader: 'TG',
			thumbnail: 'https://cdn.simplertrading.com/2025/09/29170752/MTT-TG.jpg'
		},
		{
			id: 'santas-on-his-way',
			title: "Santa's On His Way",
			date: 'December 23, 2025',
			trader: 'HG',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg'
		},
		{
			id: 'setting-up-santa-rally',
			title: 'Setting Up for the Santa Rally',
			date: 'December 22, 2025',
			trader: 'Danielle Shay',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134911/SimplerCentral_DShay.jpg'
		}
	];
</script>

<svelte:head>
	<title>{video.title} | Premium Daily Videos | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<nav class="breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span>/</span>
			<a href="/dashboard/premium-daily-videos">Premium Daily Videos</a>
			<span>/</span>
			<span class="current">{video.title}</span>
		</nav>
		<h1 class="dashboard__page-title">{video.title}</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section video-section">
			<!-- Video Player -->
			<div class="video-player">
				<div class="video-wrapper">
					<div class="video-placeholder">
						<div class="play-button">
							<svg viewBox="0 0 24 24" fill="currentColor" width="64" height="64">
								<path d="M8 5v14l11-7z" />
							</svg>
						</div>
						<img src={video.traderImage} alt={video.title} class="video-thumbnail" />
					</div>
				</div>
			</div>

			<!-- Video Info -->
			<div class="video-info">
				<h2 class="video-title">{video.title}</h2>
				<div class="video-meta">
					<span class="video-date">{video.date}</span>
					<span class="video-trader">with {video.trader}</span>
				</div>
				<div class="video-description">
					<p>{video.description}</p>
				</div>
			</div>
		</section>

		<!-- Related Videos -->
		<section class="dashboard__content-section">
			<h3 class="section-title">Related Videos</h3>
			<div class="card-grid flex-grid row">
				{#each relatedVideos as related (related.id)}
					<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-4">
						<div class="card flex-grid-panel">
							<figure class="card-media card-media--video">
								<a
									href="/dashboard/premium-daily-videos/{related.id}"
									class="card-image"
									style="background-image: url({related.thumbnail});"
								>
									<img alt={related.title} />
								</a>
							</figure>
							<section class="card-body">
								<h4 class="h5 card-title">
									<a href="/dashboard/premium-daily-videos/{related.id}">{related.title}</a>
								</h4>
								<span class="article-card__meta"
									><small>{related.date} with {related.trader}</small></span
								>
							</section>
							<footer class="card-footer">
								<a
									class="btn btn-tiny btn-default"
									href="/dashboard/premium-daily-videos/{related.id}">Watch Now</a
								>
							</footer>
						</div>
					</article>
				{/each}
			</div>
		</section>
	</div>
</div>

<style>
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid #dbdbdb;
		padding: 20px 40px;
	}

	.breadcrumb {
		font-size: 14px;
		color: #666;
		margin-bottom: 10px;
	}

	.breadcrumb a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumb a:hover {
		text-decoration: underline;
	}

	.breadcrumb span {
		margin: 0 8px;
		color: #999;
	}

	.breadcrumb .current {
		color: #333;
	}

	h1.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 32px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		display: flex;
	}

	.dashboard__content-main {
		flex: 1;
		background-color: #efefef;
	}

	.dashboard__content-section {
		padding: 30px 40px;
		background-color: #fff;
		margin-bottom: 20px;
	}

	.section-title {
		color: #333;
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 20px;
	}

	/* Video Player */
	.video-player {
		margin-bottom: 30px;
	}

	.video-wrapper {
		position: relative;
		padding-bottom: 56.25%;
		height: 0;
		overflow: hidden;
		border-radius: 8px;
		background: #000;
	}

	.video-placeholder {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		cursor: pointer;
	}

	.video-thumbnail {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0.7;
	}

	.play-button {
		position: absolute;
		width: 80px;
		height: 80px;
		background: rgba(246, 149, 50, 0.9);
		border-radius: 50%;
		display: flex;
		align-items: center;
		justify-content: center;
		color: #fff;
		transition: transform 0.2s, background 0.2s;
		z-index: 1;
	}

	.video-placeholder:hover .play-button {
		transform: scale(1.1);
		background: rgba(246, 149, 50, 1);
	}

	/* Video Info */
	.video-info {
		padding: 20px 0;
	}

	.video-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
	}

	.video-meta {
		font-size: 14px;
		color: #666;
		margin-bottom: 20px;
	}

	.video-meta span {
		margin-right: 15px;
	}

	.video-trader {
		color: #0984ae;
	}

	.video-description {
		font-size: 16px;
		line-height: 1.7;
		color: #444;
	}

	.video-description p {
		margin: 0;
	}

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-4 {
			flex: 0 0 33.333%;
			max-width: 33.333%;
		}
	}

	.card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		display: flex;
		flex-direction: column;
		height: 100%;
		transition: all 0.2s;
	}

	.card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.card-media {
		position: relative;
		height: 150px;
		overflow: hidden;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 50px;
		height: 50px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='white'%3E%3Cpath d='M8 5v14l11-7z'/%3E%3C/svg%3E");
		background-repeat: no-repeat;
		background-position: center;
		background-size: 24px;
		transition: all 0.2s;
	}

	.card-media--video:hover::after {
		background-color: rgba(246, 149, 50, 0.9);
	}

	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
	}

	.card-image img {
		opacity: 0;
	}

	.card-body {
		padding: 15px;
		flex: 1;
	}

	.card-title {
		margin: 0 0 8px;
		font-size: 16px;
		font-weight: 700;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
	}

	.card-title a:hover {
		color: #0984ae;
	}

	.article-card__meta {
		display: block;
		color: #999;
		font-size: 12px;
	}

	.card-footer {
		padding: 15px;
		border-top: 1px solid #eee;
	}

	.btn {
		display: inline-block;
		padding: 6px 14px;
		background: #f5f5f5;
		color: #333;
		border: 1px solid #ddd;
		border-radius: 4px;
		text-decoration: none;
		font-size: 12px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn:hover {
		background: #e8e8e8;
	}
</style>
