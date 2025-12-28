<script lang="ts">
	/**
	 * Watchlist Rundown Archive - Matching Simpler Trading
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Archive of weekly watchlist video rundowns (video tab)
	 * @version 2.0.0 - December 2025
	 */

	// Archive data - matches reference structure with trader images
	const archives = [
		{
			date: 'December 22, 2025',
			trader: 'TG Watkins',
			slug: '12222025-tg-watkins',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			description: 'Week of December 22, 2025.'
		},
		{
			date: 'December 15, 2025',
			trader: 'Allison Ostrander',
			slug: '12152025-allison-ostrander',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			description: 'Week of December 15, 2025.'
		},
		{
			date: 'December 08, 2025',
			trader: 'Taylor Horton',
			slug: '12082025-taylor-horton',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Taylor-Watchlist-Rundown.jpg',
			description: 'Week of December 8, 2025.'
		},
		{
			date: 'December 01, 2025',
			trader: 'Raghee Horner',
			slug: '12012025-raghee-horner',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Raghee-Watchlist-Rundown.jpg',
			description: 'Week of December 1, 2025.'
		},
		{
			date: 'November 24, 2025',
			trader: 'David Starr',
			slug: '11242025-david-starr',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			description: 'Week of November 24, 2025.'
		},
		{
			date: 'November 17, 2025',
			trader: 'Mike Teeto',
			slug: '11172025-mike-teeto',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/WeeklyWatchlist_MT.jpg',
			description: 'Week of November 17, 2025.'
		},
		{
			date: 'November 10, 2025',
			trader: 'Taylor Horton',
			slug: '11102025-taylor-horton',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Taylor-Watchlist-Rundown.jpg',
			description: 'Week of November 10, 2025.'
		},
		{
			date: 'November 03, 2025',
			trader: 'Allison Ostrander',
			slug: '11032025-allison-ostrander',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			description: 'Week of November 3, 2025.'
		},
		{
			date: 'October 27, 2025',
			trader: 'Bruce Marshall',
			slug: '10272025-bruce-marshall',
			image: 'https://cdn.simplertrading.com/2021/09/20120627/Bruce-Watchlist-Rundown.jpg',
			description: 'Week of October 27, 2025.'
		}
	];
</script>

<svelte:head>
	<title>Watchlist Rundown Archive - Revolution Trading Pros</title>
	<meta name="description" content="Archive of weekly watchlist video rundowns" />
</svelte:head>

<!-- Dashboard Layout -->
<div class="dashboard">
	<!-- Sidebar Navigation -->
	<aside class="dashboard__sidebar">
		<nav class="dashboard__nav-secondary">
			<ul>
				<li>
					<a href="/dashboard/ww/">
						<span class="dashboard__nav-item-icon st-icon-dashboard"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist</span>
					</a>
				</li>
				<li class="is-active">
					<a href="/dashboard/ww/watchlist-rundown-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Watchlist Rundown Archive</span>
					</a>
				</li>
				<li>
					<a href="/dashboard/ww/weekly-watchlist-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist Archive</span>
					</a>
				</li>
			</ul>
		</nav>
	</aside>

	<!-- Main Content -->
	<main class="dashboard__main">
		<header class="dashboard__header">
			<div class="dashboard__header-left">
				<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
			</div>
		</header>

		<div class="dashboard__content">
			<div class="dashboard__content-main">
				<div class="dashboard__content-section">
					<section>
						<h2 class="section-title">Watchlist Rundown Archive</h2>
						<div class="article-cards row flex-grid">
							{#each archives as item}
								<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
									<article class="article-card">
										<figure class="weekly_watchlist">
											<div class="article-card__image" style="background-image: url({item.image});">
												<a href="/watchlist/{item.slug}">
													<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="Weekly Watchlist with {item.trader}">
												</a>
											</div>
											<div class="card_content u--margin-top-0">
												<div>
													<h4 class="h5 article-card__title">
														<a href="/watchlist/{item.slug}">Weekly Watchlist with {item.trader}</a>
													</h4>
													<div class="u--margin-top-0 u--margin-bottom-20">
														<span><i>{item.date}</i></span><br>
													</div>
													<p class="u--margin-bottom-20">{item.description}</p>
												</div>
												<a class="watch-now-link" href="/watchlist/{item.slug}">Watch Now</a>
											</div>
										</figure>
									</article>
								</div>
							{/each}
						</div>
					</section>
				</div>
			</div>
		</div>

		<!-- Current Watchlist Section -->
		<div class="dashboard__content-section u--background-color-white">
			<section>
				<div class="row">
					<div class="col-sm-6 col-lg-5">
						<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
						<div class="hidden-md d-lg-none pb-2">
							<a href="/watchlist/12222025-tg-watkins">
								<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist image" class="u--border-radius" />
							</a>
						</div>
						<h4 class="h5 u--font-weight-bold">Weekly Watchlist with TG Watkins</h4>
						<div class="u--hide-read-more">
							<p>Week of December 22, 2025.</p>
						</div>
						<a href="/watchlist/12222025-tg-watkins" class="btn btn-tiny btn-default">Watch Now</a>
					</div>
					<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
						<a href="/watchlist/12222025-tg-watkins">
							<img src="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg" alt="Weekly Watchlist" class="u--border-radius" />
						</a>
					</div>
				</div>
			</section>
		</div>
	</main>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   Watchlist Rundown Archive - Matching Simpler Trading Exactly
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Layout */
	.dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* Sidebar - ICT 11 canonical: 220px width, #0e2433 background */
	.dashboard__sidebar {
		width: 220px;
		background: #0e2433;
		flex-shrink: 0;
	}

	.dashboard__nav-secondary ul {
		list-style: none;
		margin: 0;
		padding: 20px 0;
	}

	.dashboard__nav-secondary li {
		margin: 0;
	}

	.dashboard__nav-secondary a {
		display: flex;
		align-items: center;
		gap: 12px;
		padding: 14px 24px;
		color: #999;
		text-decoration: none;
		font-size: 14px;
		transition: all 0.2s;
	}

	.dashboard__nav-secondary a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	.dashboard__nav-secondary li.is-active a {
		color: #fff;
		background: rgba(9, 132, 174, 0.2);
		border-left: 3px solid #0984ae;
	}

	.dashboard__nav-item-icon {
		font-size: 18px;
	}

	/* Main Content */
	.dashboard__main {
		flex: 1;
		min-width: 0;
	}

	.dashboard__header {
		padding: 20px 30px;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		padding: 30px;
	}

	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 20px;
		margin-bottom: 20px;
	}

	/* Section Title */
	.section-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		padding-bottom: 15px;
		border-bottom: 2px solid #0984ae;
	}

	/* Article Cards Grid - Matching Reference */
	.article-cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -10px;
	}

	.flex-grid-item {
		padding: 10px;
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
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* Article Card */
	.article-card {
		background: #fff;
		border-radius: 4px;
		overflow: hidden;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		height: 100%;
	}

	.weekly_watchlist {
		margin: 0;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.article-card__image {
		position: relative;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center;
		background-color: #1a1a1a;
	}

	.article-card__image a {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		display: block;
	}

	.article-card__image img {
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.card_content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.card_content > div {
		flex: 1;
	}

	.article-card__title {
		font-size: 18px;
		font-weight: 600;
		margin: 0 0 10px;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.u--margin-top-0 {
		margin-top: 0;
	}

	.u--margin-bottom-20 {
		margin-bottom: 20px;
	}

	.card_content span i {
		color: #666;
		font-style: italic;
	}

	.card_content p {
		color: #666;
		font-size: 14px;
		margin: 0;
	}

	.watch-now-link {
		color: #f7941d;
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.watch-now-link:hover {
		text-decoration: underline;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		text-decoration: none;
		border-radius: 3px;
		font-weight: 600;
		transition: all 0.2s;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 13px;
	}

	.btn-default {
		background: #f5f5f5;
		color: #333;
		border: 1px solid #dbdbdb;
	}

	.btn-default:hover {
		background: #e8e8e8;
		border-color: #999;
	}

	/* Current Watchlist Section */
	.u--background-color-white {
		background: #fff;
		margin: 0 30px 30px;
		padding: 30px;
		border-radius: 4px;
	}

	.section-title-alt {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 2px solid #0984ae;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--border-radius {
		border-radius: 4px;
		width: 100%;
	}

	.row {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.col-lg-5 {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
		padding: 0 15px;
	}

	.col-lg-7 {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
		padding: 0 15px;
	}

	.pb-2 {
		padding-bottom: 10px;
	}

	/* Responsive */
	@media (max-width: 992px) {
		.dashboard__sidebar {
			display: none;
		}

		.col-lg-5,
		.col-lg-7 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.d-lg-block {
			display: none !important;
		}

		.d-lg-none {
			display: block !important;
		}
	}

	@media (max-width: 768px) {
		.dashboard__content {
			padding: 15px;
		}

		.u--background-color-white {
			margin: 0 15px 15px;
			padding: 20px;
		}
	}

	/* Hide classes */
	.hidden-xs,
	.hidden-sm {
		display: block;
	}

	.hidden-md {
		display: none;
	}

	@media (max-width: 992px) {
		.hidden-md {
			display: block;
		}

		.d-none {
			display: none !important;
		}
	}
</style>
