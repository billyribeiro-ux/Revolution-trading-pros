<script lang="ts">
	/**
	 * Weekly Watchlist Dashboard - Svelte 5 Component-Based Architecture
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main dashboard for Weekly Watchlist with featured cards and recent rundowns.
	 * Refactored to use modular Svelte 5 components.
	 *
	 * @version 2.0.0 - Svelte 5 with component-based architecture
	 */
	import {
		DashboardHeader,
		ArticleCard,
		WeeklyWatchlistSection,
		SectionTitle
	} from '$lib/components/dashboard';

	// Recent rundown archives
	const recentRundowns = [
		{
			id: 1,
			date: 'December 22, 2025',
			trader: 'TG Watkins',
			slug: '12222025-tg-watkins',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg',
			description: 'Week of December 22, 2025.'
		},
		{
			id: 2,
			date: 'December 15, 2025',
			trader: 'Allison Ostrander',
			slug: '12152025-allison-ostrander',
			image: 'https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/Allison-Watchlist-Rundown.jpg',
			description: 'Week of December 15, 2025.'
		},
		{
			id: 3,
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

<!-- DASHBOARD HEADER -->
<DashboardHeader title="Weekly Watchlist Dashboard" showRules={false} />

<!-- DASHBOARD CONTENT -->
<div class="dashboard__content">
	<div class="dashboard__content-main">

		<!-- FEATURED CARDS SECTION -->
		<section class="dashboard__content-section-member">
			<div class="featured-cards">
				<div class="featured-card featured-card--get-started">
					<h2 class="featured-card__title">Get Started</h2>
					<p>Learn how to best use the Weekly Watchlist and get the most out of it.</p>
					<a href="/dashboard/ww/getting-started/" class="featured-card__link">Check It Out</a>
				</div>
				<div class="featured-card featured-card--rundown">
					<h2 class="featured-card__title">Watchlist Rundown</h2>
					<p>Video reviewing the Watchlist of the Week by the Trader who created it.</p>
					<a href="/dashboard/ww/watchlist-rundown-archive/" class="featured-card__link">Check It Out</a>
				</div>
				<div class="featured-card featured-card--watchlist">
					<h2 class="featured-card__title">Weekly Watchlist</h2>
					<p>Get the complete Watchlist of the Week in a downloadable format.</p>
					<a href="/dashboard/ww/weekly-watchlist-archive/" class="featured-card__link">Check It Out</a>
				</div>
			</div>
		</section>

		<!-- WATCHLIST RUNDOWN ARCHIVE SECTION -->
		<section class="dashboard__content-section">
			<SectionTitle title="Watchlist Rundown Archive" />
			<div class="article-cards">
				{#each recentRundowns as item (item.id)}
					<div class="article-cards__item">
						<ArticleCard
							title="Weekly Watchlist with {item.trader}"
							href="/watchlist/{item.slug}"
							image={item.image}
							meta={item.date}
							excerpt={item.description}
							buttonText="Watch Now"
						/>
					</div>
				{/each}
			</div>
			<div class="view-all-link">
				<a href="/dashboard/ww/watchlist-rundown-archive/">View All Rundowns &rarr;</a>
			</div>
		</section>

		<!-- WEEKLY WATCHLIST FEATURED SECTION -->
		<WeeklyWatchlistSection
			title="Weekly Watchlist"
			featuredTitle="Weekly Watchlist with TG Watkins"
			description="Week of December 22, 2025."
			image="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg"
			href="/watchlist/12222025-tg-watkins"
		/>

	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	   PAGE-SPECIFIC STYLES
	   Component styles (header, cards, watchlist) come from components
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Content Layout */
	.dashboard__content {
		display: flex;
		flex-flow: row nowrap;
	}

	.dashboard__content-main {
		flex: 1 1 auto;
		min-width: 0;
	}

	/* Content Sections */
	.dashboard__content-section-member {
		padding: 30px 20px;
		background-color: #fff;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section-member { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.dashboard__content-section-member { padding: 40px; }
	}

	.dashboard__content-section {
		padding: 30px 20px;
		background-color: #fff;
	}

	@media (min-width: 1280px) {
		.dashboard__content-section { padding: 30px; }
	}

	@media (min-width: 1440px) {
		.dashboard__content-section { padding: 40px; }
	}

	/* Featured Cards Grid */
	.featured-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
	}

	.featured-card {
		background: linear-gradient(135deg, #0984ae 0%, #065a75 100%);
		border-radius: 8px;
		padding: 30px;
		color: #fff;
		text-align: center;
		min-height: 200px;
		display: flex;
		flex-direction: column;
		justify-content: center;
		box-shadow: 0 5px 30px rgba(0, 0, 0, 0.15);
		transition: transform 0.2s ease-in-out;
	}

	.featured-card:hover {
		transform: translateY(-2px);
	}

	.featured-card--get-started {
		background: linear-gradient(135deg, #28a745 0%, #1e7e34 100%);
	}

	.featured-card--rundown {
		background: linear-gradient(135deg, #0984ae 0%, #065a75 100%);
	}

	.featured-card--watchlist {
		background: linear-gradient(135deg, #F69532 0%, #dc7309 100%);
	}

	.featured-card__title {
		font-size: 24px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		margin: 0 0 15px;
	}

	.featured-card p {
		font-size: 14px;
		line-height: 1.5;
		margin: 0 0 20px;
		opacity: 0.9;
	}

	.featured-card__link {
		display: inline-block;
		background: rgba(255, 255, 255, 0.2);
		color: #fff;
		padding: 10px 20px;
		border-radius: 4px;
		text-decoration: none;
		font-weight: 700;
		font-size: 14px;
		transition: background-color 0.2s ease-in-out;
	}

	.featured-card__link:hover {
		background: rgba(255, 255, 255, 0.3);
	}

	/* Article Cards Grid */
	.article-cards {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
		gap: 20px;
		margin-bottom: 20px;
	}

	.article-cards__item {
		display: flex;
		min-width: 0;
	}

	/* View All Link */
	.view-all-link {
		text-align: right;
		margin-top: 10px;
	}

	.view-all-link a {
		color: #0984ae;
		text-decoration: none;
		font-size: 14px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: color 0.2s ease-in-out;
	}

	.view-all-link a:hover {
		color: #065a75;
		text-decoration: underline;
	}
</style>
