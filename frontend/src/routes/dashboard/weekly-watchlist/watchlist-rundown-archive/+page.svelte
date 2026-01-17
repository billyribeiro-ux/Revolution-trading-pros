<!--
	Watchlist Rundown Archive - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	Matches: frontend/Implementation/wwl-Rundown-Archive
	@version 2.0.0 - January 2026
-->
<script lang="ts">
	// 9 cards matching WordPress exactly
	const videos = [
		{
			id: '1',
			title: 'Weekly Watchlist with TG Watkins',
			date: 'January 12, 2026',
			description: 'Week of January 12, 2026.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			href: '/watchlist/01122026-tg-watkins'
		},
		{
			id: '2',
			title: 'Weekly Watchlist with Melissa Beegle',
			date: 'January 03, 2026',
			description: 'Week starting on January 3, 2026.',
			image: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			href: '/watchlist/01032026-melissa-beegle'
		},
		{
			id: '3',
			title: 'Weekly Watchlist with David Starr',
			date: 'December 29, 2025',
			description: 'Week of December 29, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			href: '/watchlist/12292025-david-starr'
		},
		{
			id: '4',
			title: 'Weekly Watchlist with TG Watkins',
			date: 'December 22, 2025',
			description: 'Week of December 22, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			href: '/watchlist/12222025-tg-watkins'
		},
		{
			id: '5',
			title: 'Weekly Watchlist with Allison Ostrander',
			date: 'December 15, 2025',
			description: 'Week of December 15, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			href: '/watchlist/12152025-allison-ostrander'
		},
		{
			id: '6',
			title: 'Weekly Watchlist with Taylor Horton',
			date: 'December 08, 2025',
			description: 'Week of December 8, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Taylor-Watchlist-Rundown.jpg',
			href: '/watchlist/12082025-taylor-horton'
		},
		{
			id: '7',
			title: 'Weekly Watchlist with Raghee Horner',
			date: 'December 01, 2025',
			description: 'Week of December 1, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Raghee-Watchlist-Rundown.jpg',
			href: '/watchlist/12012025-raghee-horner'
		},
		{
			id: '8',
			title: 'Weekly Watchlist with David Starr',
			date: 'November 24, 2025',
			description: 'Week of November 24, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			href: '/watchlist/11242025-david-starr'
		},
		{
			id: '9',
			title: 'Weekly Watchlist with Mike Teeto',
			date: 'November 17, 2025',
			description: 'Week of November 17, 2025.',
			image:
				'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/WeeklyWatchlist_MT.jpg',
			href: '/watchlist/11172025-mike-teeto'
		}
	];

	const currentPage = 1;
	const totalPages = 29;
</script>

<svelte:head>
	<title>Watchlist Rundown Archive - Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
</header>

<div class="dashboard__content-section">
	<section>
		<h2 class="section-title">Watchlist Rundown Archive</h2>
		<div class="article-cards row flex-grid">
			{#each videos as video (video.id)}
				<div class="col-xs-12 col-sm-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure class="weekly_watchlist">
							<div class="article-card__image" style="background-image: url({video.image});">
								<a href={video.href}>
									<img
										src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg"
										alt={video.title}
									/>
								</a>
							</div>
							<div class="card_content">
								<div>
									<h4 class="h5 article-card__title"><a href={video.href}>{video.title}</a></h4>
									<div class="article-card__date">
										<span><i>{video.date}</i></span>
									</div>
									<p class="article-card__description">{video.description}</p>
								</div>
								<a class="watch-now-link" href={video.href}>Watch Now</a>
							</div>
						</figure>
					</article>
				</div>
			{/each}
		</div>
	</section>

	<!-- Pagination - WordPress Match -->
	<div class="text-center">
		<div class="pagination">
			<span aria-current="page" class="page-numbers current">{currentPage}</span>
			<a class="page-numbers" href="/dashboard/weekly-watchlist/watchlist-rundown-archive?page=2"
				>2</a
			>
			<span class="page-numbers dots">&hellip;</span>
			<a class="page-numbers" href="/dashboard/weekly-watchlist/watchlist-rundown-archive?page=28"
				>28</a
			>
			<a class="page-numbers" href="/dashboard/weekly-watchlist/watchlist-rundown-archive?page=29"
				>{totalPages}</a
			>
			<a
				class="next page-numbers"
				href="/dashboard/weekly-watchlist/watchlist-rundown-archive?page=2">&raquo;</a
			>
		</div>
	</div>
</div>

<style>
	/* Dashboard Header */
	.dashboard__header {
		padding: 20px 30px;
		background: #f5f5f5;
	}

	.dashboard__page-title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0;
	}

	/* Content Section */
	.dashboard__content-section {
		padding: 20px 30px;
		background: #f5f5f5;
	}

	.section-title {
		font-size: 1.5rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
		color: #1a1a1a;
	}

	/* Article Cards Grid - WordPress Match */
	.article-cards.row.flex-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.flex-grid-item {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333333%;
		}
	}

	/* Article Card - WordPress Match */
	.article-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 3px 6px rgba(0, 0, 0, 0.16);
		height: 100%;
		display: flex;
		flex-direction: column;
		transition:
			transform 0.3s ease,
			box-shadow 0.3s ease;
		will-change: transform;
	}

	.article-card:hover {
		transform: translateY(-8px);
		box-shadow:
			0 12px 24px rgba(0, 0, 0, 0.15),
			0 4px 8px rgba(0, 0, 0, 0.1);
	}

	.weekly_watchlist {
		margin: 0;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	/* Card Image */
	.article-card__image {
		position: relative;
		width: 100%;
		padding-bottom: 56.25%;
		background-size: cover;
		background-position: center top;
		overflow: hidden;
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
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	/* Card Content - Centered */
	.card_content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		align-items: center;
		text-align: center;
		flex: 1;
	}

	.card_content > div {
		width: 100%;
		text-align: center;
	}

	.article-card__title {
		font-size: 1rem;
		font-weight: 600;
		margin: 0 0 0 0;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #1a1a1a;
		text-decoration: none;
	}

	.article-card__title a:hover {
		color: #143e59;
	}

	.article-card__date {
		margin: 0 0 20px 0;
	}

	.article-card__date span i {
		font-style: italic;
		color: #666;
		font-size: 0.875rem;
	}

	.article-card__description {
		font-size: 0.9375rem;
		line-height: 1.6;
		color: #666;
		margin: 0 0 20px 0;
	}

	.watch-now-link {
		display: inline-block;
		background: #f69532;
		color: #fff;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		border: 1px solid #f69532;
		transition: all 0.2s ease;
	}

	.watch-now-link:hover {
		background: #dc7309;
		border-color: #dc7309;
		color: #fff;
	}

	/* Pagination - WordPress Exact Match */
	.text-center {
		text-align: center;
		margin-top: 20px;
	}

	.pagination {
		display: inline-flex;
		align-items: center;
		gap: 0;
	}

	.page-numbers {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 40px;
		height: 40px;
		padding: 0 12px;
		font-size: 15px;
		font-weight: 600;
		color: #666;
		background: #fff;
		border: 1px solid #ddd;
		margin-left: -1px;
		text-decoration: none;
	}

	.page-numbers:first-child {
		border-radius: 4px 0 0 4px;
		margin-left: 0;
	}

	.page-numbers:last-child {
		border-radius: 0 4px 4px 0;
	}

	.page-numbers.current {
		background: #143e59;
		color: #fff;
		border-color: #143e59;
	}

	.page-numbers.dots {
		background: transparent;
		border: none;
		min-width: 30px;
		padding: 0 8px;
	}

	.page-numbers:hover:not(.current):not(.dots) {
		background: #f5f5f5;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header,
		.dashboard__content-section {
			padding: 15px 20px;
		}

		.page-numbers {
			min-width: 35px;
			height: 35px;
			padding: 0 8px;
			font-size: 14px;
		}
	}
</style>
