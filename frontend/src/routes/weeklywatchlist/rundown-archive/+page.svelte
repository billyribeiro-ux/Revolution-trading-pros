<script lang="ts">
	/**
	 * Weekly Watchlist Rundown Archive - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Archive of weekly watchlist video rundowns
	 * @version 1.0.0 - Pixel Perfect
	 */

	// Video archives data
	const videoArchives = [
		{ id: 1, title: 'Weekly Watchlist Rundown - December 16-20', date: 'December 15, 2025', duration: '18:24', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-1.jpg', slug: 'dec-16-20-2025' },
		{ id: 2, title: 'Weekly Watchlist Rundown - December 9-13', date: 'December 8, 2025', duration: '16:42', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-2.jpg', slug: 'dec-9-13-2025' },
		{ id: 3, title: 'Weekly Watchlist Rundown - December 2-6', date: 'December 1, 2025', duration: '15:18', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-3.jpg', slug: 'dec-2-6-2025' },
		{ id: 4, title: 'Weekly Watchlist Rundown - November 25-29', date: 'November 24, 2025', duration: '12:55', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-4.jpg', slug: 'nov-25-29-2025' },
		{ id: 5, title: 'Weekly Watchlist Rundown - November 18-22', date: 'November 17, 2025', duration: '17:30', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-5.jpg', slug: 'nov-18-22-2025' },
		{ id: 6, title: 'Weekly Watchlist Rundown - November 11-15', date: 'November 10, 2025', duration: '14:22', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-6.jpg', slug: 'nov-11-15-2025' },
		{ id: 7, title: 'Weekly Watchlist Rundown - November 4-8', date: 'November 3, 2025', duration: '16:08', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-7.jpg', slug: 'nov-4-8-2025' },
		{ id: 8, title: 'Weekly Watchlist Rundown - October 28 - November 1', date: 'October 27, 2025', duration: '15:45', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-8.jpg', slug: 'oct-28-nov-1-2025' },
		{ id: 9, title: 'Weekly Watchlist Rundown - October 21-25', date: 'October 20, 2025', duration: '18:12', thumbnail: 'https://cdn.simplertrading.com/thumbnails/watchlist-thumb-9.jpg', slug: 'oct-21-25-2025' }
	];
</script>

<svelte:head>
	<title>Weekly Watchlist Rundown Archive | Revolution Trading Pros</title>
	<meta name="description" content="Archive of weekly watchlist video rundowns with market analysis and trading insights." />
</svelte:head>

<!-- WordPress EXACT Structure -->
<div class="page-template-default page logged-in woocommerce-page">

	<!-- Breadcrumbs -->
	<nav id="breadcrumbs" class="breadcrumbs">
		<div class="container-fluid">
			<ul>
				<li class="item-home"><a class="breadcrumb-link breadcrumb-home" href="/" title="Home">Home</a></li>
				<li class="separator separator-home"> / </li>
				<li class="item-dashboard"><a class="breadcrumb-link" href="/dashboard" title="Dashboard">Dashboard</a></li>
				<li class="separator"> / </li>
				<li class="item-watchlist"><a class="breadcrumb-link" href="/weeklywatchlist" title="Weekly Watchlist">Weekly Watchlist</a></li>
				<li class="separator"> / </li>
				<li class="item-current"><strong class="breadcrumb-current">Rundown Archive</strong></li>
			</ul>
		</div>
	</nav>

	<!-- Page Content -->
	<div id="page" class="hfeed site grid-parent">
		<div id="content" class="site-content">

			<!-- Page Header -->
			<header class="page-header-section">
				<div class="container-fluid">
					<h1 class="page-title">Watchlist Rundown Archive</h1>
					<p class="page-subtitle">Video archive of weekly watchlist rundowns with TG Watkins</p>
				</div>
			</header>

			<!-- Main Content -->
			<section class="wl-page-content">
				<div class="container-fluid">
					<!-- Back Link -->
					<div class="back-link-wrap">
						<a href="/weeklywatchlist" class="back-link">
							<svg viewBox="0 0 24 24" width="16" height="16"><path fill="currentColor" d="M20 11H7.83l5.59-5.59L12 4l-8 8 8 8 1.41-1.41L7.83 13H20v-2z"/></svg>
							Back to Current Watchlist
						</a>
					</div>

					<!-- Video Grid -->
					<div class="video-grid">
						{#each videoArchives as video}
							<article class="video-card">
								<div class="video-thumbnail">
									<div class="thumbnail-image" style="background-image: url({video.thumbnail})"></div>
									<div class="play-overlay">
										<svg viewBox="0 0 24 24" width="48" height="48">
											<path fill="currentColor" d="M8 5v14l11-7z"/>
										</svg>
									</div>
									<span class="duration-badge">{video.duration}</span>
								</div>
								<div class="video-info">
									<h3 class="video-title">{video.title}</h3>
									<p class="video-date">{video.date}</p>
									<a href="/weeklywatchlist/rundown-archive/{video.slug}" class="watch-link">
										Watch Now
										<svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M8.59 16.59L13.17 12 8.59 7.41 10 6l6 6-6 6-1.41-1.41z"/></svg>
									</a>
								</div>
							</article>
						{/each}
					</div>

					<!-- Pagination -->
					<div class="pagination">
						<span class="page-info">Showing 1-9 of 52 videos</span>
						<div class="page-controls">
							<button class="page-btn" disabled>Previous</button>
							<button class="page-btn page-btn-active">1</button>
							<button class="page-btn">2</button>
							<button class="page-btn">3</button>
							<span class="page-dots">...</span>
							<button class="page-btn">6</button>
							<button class="page-btn">Next</button>
						</div>
					</div>
				</div>
			</section>

		</div>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   100% PIXEL-PERFECT STYLES - Matching Simpler Trading
	   ═══════════════════════════════════════════════════════════════════════════ */

	:global(body) {
		font-family: 'Open Sans', -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
		margin: 0;
		padding: 0;
		background: #f4f4f4;
	}

	/* Breadcrumbs */
	.breadcrumbs {
		background: #f4f4f4;
		padding: 12px 0;
		border-bottom: 1px solid #dbdbdb;
	}

	.breadcrumbs ul {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
	}

	.breadcrumbs li {
		font-size: 14px;
		color: #666;
	}

	.breadcrumbs a {
		color: #0984ae;
		text-decoration: none;
	}

	.breadcrumbs a:hover {
		text-decoration: underline;
	}

	.separator {
		margin: 0 8px;
	}

	/* Container */
	.container-fluid {
		max-width: 1200px;
		margin: 0 auto;
		padding: 0 20px;
	}

	/* Page Header */
	.page-header-section {
		padding: 40px 0 30px;
	}

	.page-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 36px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
		text-transform: uppercase;
	}

	.page-subtitle {
		color: #666;
		font-size: 16px;
		font-weight: 500;
		margin: 0;
	}

	/* Back Link */
	.back-link-wrap {
		margin-bottom: 24px;
	}

	.back-link {
		display: inline-flex;
		align-items: center;
		gap: 8px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
	}

	.back-link:hover {
		color: #076787;
		text-decoration: underline;
	}

	/* Video Grid */
	.video-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
		gap: 24px;
		margin-bottom: 40px;
	}

	.video-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.video-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	.video-thumbnail {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background: #1a1a1a;
		overflow: hidden;
	}

	.thumbnail-image {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		background-color: #0984ae;
		transition: transform 0.3s ease;
	}

	.video-card:hover .thumbnail-image {
		transform: scale(1.05);
	}

	.play-overlay {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(0, 0, 0, 0.3);
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.video-card:hover .play-overlay {
		opacity: 1;
	}

	.play-overlay svg {
		width: 64px;
		height: 64px;
		color: #fff;
		filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.3));
	}

	.duration-badge {
		position: absolute;
		bottom: 10px;
		right: 10px;
		background: rgba(0, 0, 0, 0.8);
		color: #fff;
		padding: 4px 8px;
		border-radius: 4px;
		font-size: 12px;
		font-weight: 600;
	}

	.video-info {
		padding: 20px;
	}

	.video-title {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 8px;
		line-height: 1.4;
	}

	.video-date {
		font-size: 13px;
		color: #666;
		margin: 0 0 12px;
	}

	.watch-link {
		display: inline-flex;
		align-items: center;
		gap: 4px;
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 600;
	}

	.watch-link:hover {
		color: #076787;
	}

	/* Pagination */
	.pagination {
		display: flex;
		align-items: center;
		justify-content: space-between;
		flex-wrap: wrap;
		gap: 16px;
		padding: 20px 0 40px;
	}

	.page-info {
		font-size: 14px;
		color: #666;
	}

	.page-controls {
		display: flex;
		align-items: center;
		gap: 8px;
	}

	.page-btn {
		padding: 8px 14px;
		background: #fff;
		color: #333;
		border: 1px solid #dbdbdb;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 500;
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.page-btn:hover:not(:disabled) {
		background: #f4f4f4;
		border-color: #999;
	}

	.page-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.page-btn-active {
		background: #0984ae;
		color: #fff;
		border-color: #0984ae;
	}

	.page-dots {
		padding: 0 8px;
		color: #666;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.page-title {
			font-size: 28px;
		}

		.video-grid {
			grid-template-columns: 1fr;
		}

		.pagination {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
