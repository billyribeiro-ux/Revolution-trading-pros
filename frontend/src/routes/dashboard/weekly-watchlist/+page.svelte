<!--
	Weekly Watchlist Dashboard - Pixel-Perfect WordPress Match
	═══════════════════════════════════════════════════════════════════════════
	
	Apple ICT 11+ Principal Engineer Grade - January 2026
	Svelte 5 Best Practices - Latest Nov/Dec 2025 Syntax
	
	@version 2.1.0 - January 2026 - Semantic class names, Svelte 5 patterns
-->
<script lang="ts">
	import FeaturedCards from '$lib/components/dashboard/FeaturedCards.svelte';
	import RightSidebar from '$lib/components/dashboard/RightSidebar.svelte';
	
	// Type definitions for component data
	interface FeaturedCard {
		id: number;
		title: string;
		description: string;
		href: string;
		bgImage: string;
		cardClass: string;
		titleClass: string;
	}
	
	interface ArchiveItem {
		id: number;
		title: string;
		date: string;
		weekOf: string;
		image: string;
		href: string;
	}

	// Featured cards configuration
	const cards = [
		{
			id: 1,
			title: 'Get Started',
			description: 'Learn how to best use the Weekly Watchlist and get the most out of it.',
			href: '/dashboard/weekly-watchlist/getting-started/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134809/getting-started-top-card-bg-1.png',
			cardClass: 'img_1',
			titleClass: 'card_title'
		},
		{
			id: 2,
			title: 'Watchlist Rundown',
			description: 'Video reviewing the Watchlist of the Week by the Trader who created it.',
			href: '/dashboard/weekly-watchlist/watchlist-rundown-archive/',
			bgImage: 'https://cdn.simplertrading.com/2021/09/30134811/watchlist-rundown-top-card-bg.png',
			cardClass: 'img_2',
			titleClass: 'card_title_2'
		},
		{
			id: 3,
			title: 'Weekly Watchlist',
			description: 'Get the complete Watchlist of the Week in a downloadable format.',
			href: '/dashboard/weekly-watchlist/archive/',
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

	// Sidebar configuration - matches WordPress reference
	const sidebarSections = [
		{
			heading: 'Weekly Watchlist Schedule',
			subheading: 'Schedule is subject to change.',
			content: 'schedule' as const
		}
	];
</script>

<svelte:head>
	<title>Weekly Watchlist Dashboard | Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
	</div>
</header>

<div class="dashboard__content">

		<div class="dashboard__content-main">
		
		<!-- FEATURED CARDS SECTION - 2x1 Layout (2 top, 1 centered bottom) -->
		<FeaturedCards {cards} layout="2x1" />

	<!-- WATCHLIST RUNDOWN ARCHIVE SECTION -->
	<div class="dashboard__content-section">
		<section>
			<h2 class="section-title">Watchlist Rundown Archive</h2>
			<div class="video-grid">
				{#each archiveItems as { id, title, date, weekOf, image, href } (id)}
					<div class="video-grid__item">
						<article class="video-card">
							<figure class="video-card__figure">
								<div class="video-card__thumbnail" style="background-image: url({image});">
									<a {href}>
										<img src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" alt={title} loading="lazy" />
									</a>
								</div>
								<div class="video-card__content">
									<div>
										<h4 class="video-card__title">
											<a {href}>{title}</a>
										</h4>
										<div class="video-card__date">
											<span><i>{date}</i></span><br />
										</div>
										<p class="video-card__description">{weekOf}</p>
									</div>
									<a {href} class="video-card__link">Watch Now</a>
								</div>
							</figure>
						</article>
					</div>
				{/each}
			</div>
		</section>
	</div>

	</div>

	<!-- SIDEBAR - Using RightSidebar component (SSOT) -->
	<RightSidebar sections={sidebarSections} />

</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT LAYOUT - Apple ICT 7 Fix
	 * Exact match to Day Trading Room (working reference)
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		border-right: 1px solid #dbdbdb;
		flex: 1 1 auto;
		min-width: 0;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * SIDEBAR - Apple ICT 7 Fix: Parent must style child component
	 * RightSidebar component CSS is scoped, so parent needs :global()
	 * ═══════════════════════════════════════════════════════════════════════════ */
	:global(.dashboard__content-sidebar) {
		display: none;
		width: 260px;
		flex: 0 0 auto;
		margin-top: -1px;
		background: #fff;
		border-right: 1px solid #dbdbdb;
		border-top: 1px solid #dbdbdb;
		font-family: 'Open Sans', sans-serif;
		font-size: 14px;
		line-height: 1.6;
	}

	@media (min-width: 1080px) {
		:global(.dashboard__content-sidebar) {
			display: block;
		}
	}

	:global(.content-sidebar__section) {
		padding: 20px 30px 20px 20px;
		border-bottom: 1px solid #dbdbdb;
	}

	:global(.content-sidebar__heading) {
		padding: 15px 20px;
		margin: -20px -30px 20px -20px;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		letter-spacing: 0.1em;
		text-transform: uppercase;
		color: #333;
		background: #ededed;
		border-bottom: 1px solid #dbdbdb;
		line-height: 1.4;
	}

	:global(.content-sidebar__heading .text-center) {
		text-align: center;
	}

	:global(.pssubject) {
		font-size: 10px;
		margin-top: 15px;
		text-transform: initial;
		font-weight: 400;
		letter-spacing: normal;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * VIDEO GRID - Archive Section
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.section-title {
		color: #333;
		font-weight: 700;
		font-size: 20px;
		margin-bottom: 30px;
		font-family: 'Open Sans', sans-serif;
		line-height: 1.2;
	}

	.video-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.video-grid__item {
		display: flex;
		margin-bottom: 30px;
		width: 100%;
		padding: 0 15px;
		box-sizing: border-box;
	}

	@media (min-width: 576px) {
		.video-grid__item {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.video-grid__item {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	.video-card {
		background: #fff;
		border-radius: 5px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
		transition: all 0.3s ease;
		display: flex;
		flex-direction: column;
		width: 100%;
	}

	.video-card:hover {
		box-shadow: 0 8px 16px rgba(20, 62, 89, 0.12), 0 16px 32px rgba(20, 62, 89, 0.08);
		transform: scale(1.01);
	}

	.video-card__figure {
		margin: 0;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.video-card__thumbnail {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		overflow: hidden;
	}

	.video-card__thumbnail img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.video-card__content {
		padding: 20px;
		display: flex;
		flex-direction: column;
		flex-grow: 1;
		align-items: center;
		text-align: center;
	}

	.video-card__content > div {
		width: 100%;
		text-align: center;
	}

	.video-card__title {
		margin: 0 0 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.video-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.video-card__title a:hover {
		color: #143E59;
	}

	.video-card__date {
		margin-bottom: 20px;
	}

	.video-card__date span {
		color: #666;
		font-size: 13px;
		line-height: 1.6;
	}

	.video-card__date span i {
		font-style: italic;
		color: #666;
	}

	.video-card__description {
		color: #666;
		font-size: 14px;
		line-height: 1.6;
		margin: 0 0 20px;
	}

	.video-card__link {
		margin-top: auto;
		display: inline-block;
		background: #F69532;
		color: #fff;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		border: 1px solid #F69532;
		transition: all 0.2s ease;
	}

	.video-card__link:hover {
		background: #dc7309;
		border-color: #dc7309;
		color: #fff;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * CONTENT SECTIONS
	 * ═══════════════════════════════════════════════════════════════════════════ */
	.dashboard__content-section {
		padding: 30px 20px;
		overflow-x: auto;
		overflow-y: hidden;
	}

	@media screen and (min-width: 1024px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	@media screen and (min-width: 1440px) {
		.dashboard__content-section {
			padding: 40px;
		}
	}

</style>
