<script lang="ts">
	/**
	 * Weekly Watchlist Dashboard - Main Page
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Main dashboard for Weekly Watchlist with featured cards and recent rundowns
	 * Matches Simpler Trading's exact layout
	 * @version 1.0.0 - December 2025
	 */

	// Recent rundown archives (first 3)
	const recentRundowns = [
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
		}
	];
</script>

<svelte:head>
	<title>Weekly Watchlist Dashboard - Revolution Trading Pros</title>
	<meta name="description" content="Get Started with Weekly Watchlist. Video rundowns and downloadable watchlists each week." />
</svelte:head>

<!-- Dashboard Layout -->
<div class="dashboard">
	<!-- Sidebar Navigation -->
	<aside class="dashboard__sidebar">
		<nav class="dashboard__nav-secondary">
			<ul>
				<li class="is-active">
					<a href="/dashboard/ww/">
						<span class="dashboard__nav-item-icon st-icon-dashboard"></span>
						<span class="dashboard__nav-item-text">Weekly Watchlist</span>
					</a>
				</li>
				<li>
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
				<!-- Featured Cards Section -->
				<section class="dashboard__content-section-member">
					<div class="row featured_cards">
						<div class="col-md-6 col-lg-4 text-center">
							<div class="featured-card img_1">
								<h2 class="card_title">Get Started</h2>
								<p>Learn how to best use the Weekly Watchlist and get the most out of it.</p>
								<a href="/dashboard/ww/getting-started/">Check It Out</a>
							</div>
						</div>
						<div class="col-md-6 col-lg-4 text-center">
							<div class="featured-card img_2">
								<h2 class="card_title_2">Watchlist Rundown</h2>
								<p>Video reviewing the Watchlist of the Week by the Trader who created it.</p>
								<div class="buttons">
									<a href="/dashboard/ww/watchlist-rundown-archive/">Check It Out</a>
								</div>
							</div>
						</div>
						<div class="col-md-6 col-lg-4 text-center">
							<div class="featured-card img_3">
								<h2 class="card_title_3">Weekly Watchlist</h2>
								<p>Get the complete Watchlist of the Week in a downloadable format.</p>
								<a href="/dashboard/ww/weekly-watchlist-archive/">Check It Out</a>
							</div>
						</div>
					</div>
				</section>

				<!-- Watchlist Rundown Archive Section -->
				<div class="dashboard__content-section">
					<section>
						<h2 class="section-title">Watchlist Rundown Archive</h2>
						<div class="article-cards row flex-grid">
							{#each recentRundowns as item}
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
						<div class="view-all-link">
							<a href="/dashboard/ww/watchlist-rundown-archive/">View All Rundowns &rarr;</a>
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
	   Weekly Watchlist Dashboard - Matching Simpler Trading Exactly
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Layout */
	.dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* Sidebar - WordPress dashboard.8f78208b.css: 280px width, #0f2d41 background */
	.dashboard__sidebar {
		width: 280px;
		background: #0f2d41;
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

	/* Secondary nav links - WordPress reference: padding 12px 20px, color hsla(0,0%,100%,0.7) */
	.dashboard__nav-secondary a {
		display: flex;
		align-items: center;
		gap: 10px;
		padding: 12px 20px;
		color: hsla(0, 0%, 100%, 0.7);
		text-decoration: none;
		font-size: 14px;
		transition: all 0.15s ease-in-out;
	}

	.dashboard__nav-secondary a:hover {
		color: #fff;
		background: rgba(255, 255, 255, 0.05);
	}

	/* Active state - WordPress reference: rgba(255,255,255,0.1), no border */
	.dashboard__nav-secondary li.is-active a {
		color: #fff;
		background: rgba(255, 255, 255, 0.1);
	}

	.dashboard__nav-item-icon {
		font-size: 24px;
		width: 24px;
		height: 24px;
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

	/* Page title - WordPress reference: font-size 36px */
	.dashboard__page-title {
		font-size: 36px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: 'Open Sans', sans-serif;
	}

	.dashboard__content {
		padding: 30px;
	}

	/* Featured Cards Section */
	.dashboard__content-section-member {
		margin-bottom: 30px;
	}

	.featured_cards {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.featured_cards .col-md-6,
	.featured_cards .col-lg-4 {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	.featured-card {
		background: linear-gradient(135deg, #1a1a1a 0%, #333 100%);
		border-radius: 8px;
		padding: 40px 30px;
		min-height: 280px;
		display: flex;
		flex-direction: column;
		justify-content: center;
		color: #fff;
	}

	.featured-card.img_1 {
		background: linear-gradient(135deg, #0984ae 0%, #076787 100%);
	}

	.featured-card.img_2 {
		background: linear-gradient(135deg, #f7941d 0%, #d47a0d 100%);
	}

	.featured-card.img_3 {
		background: linear-gradient(135deg, #28a745 0%, #1e7e34 100%);
	}

	.featured-card h2 {
		font-size: 24px;
		font-weight: 700;
		margin: 0 0 15px;
		text-transform: uppercase;
	}

	.featured-card p {
		font-size: 15px;
		line-height: 1.6;
		margin: 0 0 20px;
		opacity: 0.9;
	}

	.featured-card a {
		display: inline-block;
		padding: 10px 24px;
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
		text-decoration: none;
		border-radius: 4px;
		font-weight: 600;
		font-size: 14px;
		transition: all 0.2s;
	}

	.featured-card a:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Dashboard Content Section */
	.dashboard__content-section {
		background: #fff;
		border-radius: 4px;
		padding: 20px;
		margin-bottom: 20px;
	}

	/* Section Title - WordPress reference: font-size 32px, margin 0 0 30px, no border */
	.section-title {
		font-size: 32px;
		font-weight: 700;
		color: #333;
		margin: 0 0 30px;
	}

	/* Article Cards Grid */
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
		.article-cards .col-md-6 {
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

	/* ═══════════════════════════════════════════════════════════════════════════
	   ARTICLE CARD - Pixel-perfect match to Simpler Trading reference
	   Reference: dashboard-globals.css lines 931-1021
	   ═══════════════════════════════════════════════════════════════════════════ */
	.article-card {
		position: relative;
		background-color: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 8px;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.1);
		transition: all 0.2s ease-in-out;
		overflow: hidden;
		margin-bottom: 30px;
		height: 100%;
	}

	.article-card:hover {
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transform: translateY(-2px);
	}

	.weekly_watchlist {
		margin: 0;
		height: 100%;
		display: flex;
		flex-direction: column;
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%; /* 16:9 aspect ratio */
		background-size: cover;
		background-position: center;
		background-color: #0984ae; /* Fallback color */
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
		padding: 15px;
		display: flex;
		flex-direction: column;
		flex: 1;
	}

	.card_content > div {
		flex: 1;
	}

	.article-card__title {
		font-size: 16px;
		font-weight: 700;
		margin: 0 0 10px;
		line-height: 1.3;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
	}

	.article-card__title a:hover {
		color: #0984ae;
	}

	.u--margin-top-0 {
		margin-top: 0;
	}

	.u--margin-bottom-20 {
		margin-bottom: 15px;
	}

	.card_content span i {
		color: #999;
		font-style: italic;
		font-size: 12px;
	}

	.card_content p {
		color: #666;
		font-size: 14px;
		line-height: 1.5;
		margin: 0;
	}

	/* Watch Now button - Orange style from reference */
	.watch-now-link {
		background: transparent;
		color: #F3911B;
		text-decoration: none;
		font-weight: 700;
		font-size: 17px;
		padding-left: 0;
		transition: all 0.2s ease-in-out;
	}

	.watch-now-link:hover {
		color: #F3911B;
		background: #e7e7e7;
		padding-left: 8px;
	}

	.view-all-link {
		margin-top: 20px;
		text-align: center;
	}

	.view-all-link a {
		color: #0984ae;
		text-decoration: none;
		font-weight: 600;
		font-size: 14px;
	}

	.view-all-link a:hover {
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
		color: #333;
		background-color: #e6e6e6;
		border-color: #adadad;
	}

	/* Current Watchlist Section */
	.u--background-color-white {
		background: #fff;
		margin: 0 30px 30px;
		padding: 30px;
		border-radius: 4px;
	}

	/* Section title alt - WordPress reference: 11px blue uppercase, letter-spacing 1px */
	.section-title-alt {
		font-size: 11px;
		font-weight: 700;
		color: #0984ae;
		margin: 0 0 20px;
		text-transform: uppercase;
		letter-spacing: 1px;
	}

	.section-title-alt--underline {
		padding-bottom: 15px;
		border-bottom: 1px solid #dbdbdb;
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

	.text-center {
		text-align: center;
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

		.featured-card {
			min-height: 220px;
			padding: 30px 20px;
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
