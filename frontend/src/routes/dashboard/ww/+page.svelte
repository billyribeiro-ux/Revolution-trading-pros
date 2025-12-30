<script lang="ts">
	/**
	 * Weekly Watchlist Dashboard - Svelte 5 Component-Based Architecture
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * Main dashboard for Weekly Watchlist with featured cards and recent rundowns.
	 * Now fetches watchlist items from API.
	 *
	 * @version 2.1.0 - Svelte 5 with watchlist API integration
	 */
	import { untrack } from 'svelte';
	import { browser } from '$app/environment';
	import {
		DashboardHeader,
		ArticleCard,
		WeeklyWatchlistSection,
		SectionTitle,
		LoadingState,
		EmptyState
	} from '$lib/components/dashboard';
	import { watchlistApi, type WatchlistItem } from '$lib/api/watchlist';

	// Watchlist data from API
	let watchlistItems = $state<WatchlistItem[]>([]);
	let isLoading = $state(true);
	let latestWatchlist = $derived(watchlistItems[0] || null);

	// Fetch watchlist items on mount
	$effect(() => {
		if (!browser) return;

		untrack(() => {
			isLoading = true;
		});

		watchlistApi.getPublished({ per_page: 5 })
			.then((response) => {
				watchlistItems = response.data || [];
				isLoading = false;
			})
			.catch((err) => {
				console.error('Failed to fetch watchlist:', err);
				isLoading = false;
			});
	});

	// Weekly Watchlist - Dynamic date calculation (fallback)
	const weeklyWatchlistDate = $derived.by(() => {
		const now = new Date();
		const day = now.getDay();
		const diff = now.getDate() - day + (day === 0 ? -6 : 1);
		const monday = new Date(now.setDate(diff));
		return monday.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
	});
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
			{#if isLoading}
				<LoadingState message="Loading watchlist items..." />
			{:else if watchlistItems.length > 0}
				<div class="article-cards">
					{#each watchlistItems.slice(0, 3) as item (item.id)}
						<div class="article-cards__item">
							<ArticleCard
								title={item.title}
								href="/watchlist/{item.slug}"
								image={item.video.poster}
								meta={item.datePosted}
								excerpt={item.subtitle}
								buttonText="Watch Now"
							/>
						</div>
					{/each}
				</div>
			{:else}
				<EmptyState title="No watchlist items available." />
			{/if}
			<div class="view-all-link">
				<a href="/dashboard/ww/watchlist-rundown-archive/">View All Rundowns &rarr;</a>
			</div>
		</section>

		<!-- WEEKLY WATCHLIST FEATURED SECTION -->
		{#if latestWatchlist}
			<WeeklyWatchlistSection
				title="Weekly Watchlist"
				featuredTitle={latestWatchlist.title}
				description={latestWatchlist.subtitle}
				image={latestWatchlist.video.poster}
				href="/watchlist/{latestWatchlist.slug}"
			/>
		{:else if !isLoading}
			<WeeklyWatchlistSection
				title="Weekly Watchlist"
				featuredTitle="Weekly Watchlist"
				description="Week of {weeklyWatchlistDate}."
				image="https://simpler-cdn.s3.amazonaws.com/azure-blob-files/weekly-watchlist/TG-Watchlist-Rundown.jpg"
				href="/watchlist/latest"
			/>
		{/if}

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
		border-radius: 4px;
		padding: 30px;
		color: #fff;
		text-align: center;
		min-height: 200px;
		display: flex;
		flex-direction: column;
		justify-content: center;
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
		transition: box-shadow 0.2s ease-in-out;
	}

	.featured-card:hover {
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
