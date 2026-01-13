<!--
	Weekly Watchlist Dashboard - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	
	Matches frontend/Implementation/wwlDashboard exactly.
	Three featured cards with background images and archive section below.
	
	@version 1.0.0 - January 2026 - WordPress pixel-perfect match
-->
<script lang="ts">
	import { onMount } from 'svelte';

	// Featured cards data matching WordPress
	const featuredCards = [
		{
			id: 1,
			title: 'Get Started',
			description: 'Learn how to best use the Weekly Watchlist and get the most out of it.',
			href: '/dashboard/ww/getting-started/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png',
			cardClass: 'img_1',
			titleClass: 'card_title'
		},
		{
			id: 2,
			title: 'Watchlist Rundown',
			description: 'Video reviewing the Watchlist of the Week by the Trader who created it.',
			href: '/dashboard/ww/watchlist-rundown-archive/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png',
			cardClass: 'img_2',
			titleClass: 'card_title_2'
		},
		{
			id: 3,
			title: 'Weekly Watchlist',
			description: 'Get the complete Watchlist of the Week in a downloadable format.',
			href: '/dashboard/ww/weekly-watchlist-archive/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134813/weekly-watchlist-top-card-bg.png',
			cardClass: 'img_3',
			titleClass: 'card_title_3'
		}
	];

	// Archive items matching WordPress
	const archiveItems = [
		{
			id: 1,
			title: 'Weekly Watchlist with TG Watkins',
			date: 'January 12, 2026',
			weekOf: 'Week of January 12, 2026.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			href: '/watchlist/01122026-tg-watkins'
		},
		{
			id: 2,
			title: 'Weekly Watchlist with Melissa Beegle',
			date: 'January 03, 2026',
			weekOf: 'Week starting on January 3, 2026.',
			image: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			href: '/watchlist/01032026-melissa-beegle'
		},
		{
			id: 3,
			title: 'Weekly Watchlist with David Starr',
			date: 'December 29, 2025',
			weekOf: 'Week of December 29, 2025.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			href: '/watchlist/12292025-david-starr'
		},
		{
			id: 4,
			title: 'Weekly Watchlist with TG Watkins',
			date: 'December 22, 2025',
			weekOf: 'Week of December 22, 2025.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			href: '/watchlist/12222025-tg-watkins'
		},
		{
			id: 5,
			title: 'Weekly Watchlist with Allison Ostrander',
			date: 'December 15, 2025',
			weekOf: 'Week starting on December 15, 2025.',
			image: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			href: '/watchlist/12152025-allison-ostrander'
		},
		{
			id: 6,
			title: 'Weekly Watchlist with Taylor Horton',
			date: 'December 08, 2025',
			weekOf: 'Week of December 8, 2025.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			href: '/watchlist/12082025-taylor-horton'
		},
		{
			id: 7,
			title: 'Weekly Watchlist with TG Watkins',
			date: 'December 01, 2025',
			weekOf: 'Week of December 1, 2025.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			href: '/watchlist/12012025-tg-watkins'
		},
		{
			id: 8,
			title: 'Weekly Watchlist with Allison Ostrander',
			date: 'November 24, 2025',
			weekOf: 'Week of November 24, 2025.',
			image: 'https://cdn.simplertrading.com/2025/03/09130833/Melissa-WeeklyWatchlist.jpg',
			href: '/watchlist/11242025-allison-ostrander'
		},
		{
			id: 9,
			title: 'Weekly Watchlist with Taylor Horton',
			date: 'November 17, 2025',
			weekOf: 'Week of November 17, 2025.',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/David-Watchlist-Rundown.jpg',
			href: '/watchlist/11172025-taylor-horton'
		}
	];

	// Sidebar watchlist dates
	const sidebarDates = [
		{ title: 'Weekly Watchlist', date: 'Jan 12, 2026' },
		{ title: 'Weekly Watchlist', date: 'Jan 5, 2026' },
		{ title: 'Weekly Watchlist', date: 'Dec 29, 2025' },
		{ title: 'Weekly Watchlist', date: 'Dec 22, 2025' },
		{ title: 'Weekly Watchlist', date: 'Dec 15, 2025' },
		{ title: 'Weekly Watchlist', date: 'Dec 8, 2025' },
		{ title: 'Weekly Watchlist', date: 'Dec 1, 2025' },
		{ title: 'Weekly Watchlist', date: 'Nov 24, 2025' },
		{ title: 'Weekly Watchlist', date: 'Nov 17, 2025' }
	];
</script>

<svelte:head>
	<title>Weekly Watchlist Dashboard | Revolution Trading Pros</title>
</svelte:head>

<main class="dashboard__main">
	
	<header class="dashboard__header">
		<div class="dashboard__header-left">
			<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
		</div>
	</header>

	<div class="dashboard__content">

		<div class="dashboard__content-main">
			
			<!-- FEATURED CARDS SECTION -->
			<section class="dashboard__content-section-member">
			<div class="row featured_cards">
				{#each featuredCards as card (card.id)}
					<div class="col-md-6 col-lg-4 text-center">
						<div class="{card.cardClass}">
							<h2 class="{card.titleClass}">{card.title}</h2>
							<p>{card.description}</p>
							<a href="{card.href}">Check It Out</a>
						</div>
					</div>
				{/each}
			</div>
		</section>

		<!-- WATCHLIST RUNDOWN ARCHIVE SECTION -->
		<div class="dashboard__content-section">
			<section>
				<h2 class="section-title">Watchlist Rundown Archive</h2>
				<div class="article-cards row flex-grid">
					{#each archiveItems as item (item.id)}
						<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
							<article class="article-card">
								<figure class="weekly_watchlist">
									<div class="article-card__image" style="background-image: url({item.image});">
										<a href="{item.href}">
											<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt="{item.title}" loading="lazy" />
										</a>
									</div>
									<div class="card_content u--margin-top-0">
										<div>
											<h4 class="h5 article-card__title">
												<a href="{item.href}">{item.title}</a>
											</h4>
											<div class="u--margin-top-0 u--margin-bottom-20">
												<span><i>{item.date}</i></span><br />
											</div>
											<p class="u--margin-bottom-20">{item.weekOf}</p>
										</div>
										<a href="{item.href}" style="color: #f7941d;">Watch Now</a>
									</div>
								</figure>
							</article>
						</div>
					{/each}
				</div>
			</section>
		</div>

	</div>

	<!-- SIDEBAR -->
	<aside class="dashboard__content-sidebar">
		<section class="content-sidebar__section">
			<h4 class="content-sidebar__heading">
				<div class="text-center">Weekly Watchlist Schedule</div>
				<p class="pssubject" style="font-size: 10px; margin-top: 15px; text-transform: initial;">Schedule is subject to change.</p>
			</h4>
			<div class="sidebar-list">
				{#each sidebarDates as item}
					<h4>{item.title}</h4>
					<span>{item.date}</span>
				{/each}
			</div>
		</section>
	</aside>

	</div>

</main>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * FEATURED CARDS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.featured_cards {
		display: flex;
		flex-wrap: wrap;
		justify-content: center;
	}

	.featured_cards .text-center {
		display: flex;
		margin-bottom: 20px;
		min-width: 384px;
		max-width: 485px;
	}

	.featured_cards p {
		padding: 0 20px;
		color: #ffffff;
		font-size: 14px;
		line-height: 1.6;
	}

	.featured_cards a {
		color: #fff !important;
		display: inline-block;
		font-family: "Open Sans Condensed", sans-serif;
		font-weight: 700;
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		border-radius: 50px;
		padding: 5px 30px;
		margin-bottom: 20px;
		text-decoration: none;
		transition: all 0.2s ease;
	}

	.featured_cards a:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
	}

	/* Card Titles */
	.card_title {
		background-color: #19416b;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	.card_title_2 {
		background-color: #45770c;
		opacity: 0.81;
		border-radius: 8px 8px 0 0;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	.card_title_3 {
		background-color: #1f9eab;
		border-radius: 8px 8px 0 0;
		opacity: 0.81;
		padding: 20px;
		font-weight: 700;
		font-size: 30px;
		color: #ffffff;
		margin: 0;
	}

	/* Card Background Images */
	.featured_cards .img_1 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png');
	}

	.featured_cards .img_2 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png');
	}

	.featured_cards .img_3 {
		width: 100%;
		border-radius: 8px;
		background-position: center;
		background-repeat: no-repeat;
		background-size: cover;
		color: #ffffff;
		background-image: url('https://cdn.simplertrading.com/2021/09/30134813/weekly-watchlist-top-card-bg.png');
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * RESPONSIVE GRID
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.col-md-6,
	.col-lg-4,
	.col-12 {
		padding-left: 15px;
		padding-right: 15px;
		width: 100%;
	}

	/* Mobile first - full width */
	.text-center {
		text-align: center;
		width: 100%;
	}

	/* Tablet - 2 columns */
	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Desktop - 3 columns */
	@media (min-width: 992px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * ARCHIVE SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.article-cards {
		display: flex;
		flex-wrap: wrap;
	}

	.flex-grid {
		display: flex;
		flex-wrap: wrap;
	}

	/* Archive Cards */
	.flex-grid-item {
		display: flex;
		margin-bottom: 20px;
	}

	.article-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: all 0.3s ease;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.article-card:hover {
		box-shadow: 0 8px 16px rgba(20, 62, 89, 0.12), 0 16px 32px rgba(20, 62, 89, 0.08);
		transform: scale(1.01);
	}

	.weekly_watchlist {
		margin: 0;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
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

	.card_content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		flex-grow: 1;
	}

	.article-card__title {
		margin: 0 0 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #143E59;
	}

	.u--margin-top-0 {
		margin-top: 0;
	}

	.u--margin-bottom-20 {
		margin-bottom: 20px;
	}

	.card_content p {
		color: #666;
		font-size: 14px;
		line-height: 1.6;
		margin: 0;
	}

	.card_content > a {
		margin-top: auto;
		font-weight: 600;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card_content > a:hover {
		color: #c67000 !important;
	}

	.col-xs-12 {
		width: 100%;
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

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT STRUCTURE
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content {
		display: flex;
		flex-wrap: wrap;
		width: 100%;
	}

	.dashboard__content-main {
		flex: 1 1 100%;
		max-width: 100%;
	}

	.dashboard__content-section {
		padding: 30px 20px;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-sidebar {
		flex: 0 0 100%;
		max-width: 100%;
		padding: 20px;
		background: #f5f5f5;
	}

	@media (min-width: 992px) {
		.dashboard__content-main {
			flex: 0 0 70%;
			max-width: 70%;
		}

		.dashboard__content-sidebar {
			flex: 0 0 30%;
			max-width: 30%;
		}
	}

	.content-sidebar__section {
		background: #fff;
		border-radius: 5px;
		padding: 20px;
		margin-bottom: 20px;
	}

	.content-sidebar__heading {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0 0 15px;
		padding-bottom: 15px;
		border-bottom: 2px solid #f5f5f5;
	}

	.content-sidebar__heading .text-center {
		text-align: center;
	}

	.pssubject {
		font-weight: 400;
		color: #666;
	}

	.sidebar-list h4 {
		font-size: 14px;
		font-weight: 600;
		color: #333;
		margin: 15px 0 5px;
	}

	.sidebar-list h4:first-child {
		margin-top: 0;
	}

	.sidebar-list span {
		display: block;
		font-size: 13px;
		color: #666;
		margin-bottom: 10px;
	}
</style>
