<script lang="ts">
	/**
	 * Premium Daily Videos Page - WordPress Revolution Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/mastering-the-trade/daily-videos
	 * Shows daily premium video content from traders.
	 *
	 * Based on SimplerPremiumDailyVideos reference structure.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import {
		IconVideo,
		IconPlayerPlay,
		IconCalendar,
		IconSearch,
		IconFilter,
		IconChevronLeft,
		IconChevronRight,
		IconUser
	} from '@tabler/icons-svelte';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// Format slug to readable title
	const membershipTitle = $derived(
		slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')
	);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedTrader = $state('all');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	// ═══════════════════════════════════════════════════════════════════════════
	// TRADERS FILTER
	// ═══════════════════════════════════════════════════════════════════════════

	const traders = [
		{ id: 'all', name: 'All Traders' },
		{ id: 'john-carter', name: 'John Carter' },
		{ id: 'henry-gambell', name: 'Henry Gambell' },
		{ id: 'danielle-shay', name: 'Danielle Shay' },
		{ id: 'bruce-marshall', name: 'Bruce Marshall' },
		{ id: 'sam-shames', name: 'Sam Shames' },
		{ id: 'allison-ostrander', name: 'Allison Ostrander' },
		{ id: 'taylor-horton', name: 'Taylor Horton' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DAILY VIDEOS (Mock data - matches SimplerPremiumDailyVideos structure)
	// ═══════════════════════════════════════════════════════════════════════════

	interface DailyVideo {
		id: number;
		title: string;
		description: string;
		date: string;
		trader: string;
		traderId: string;
		thumbnail: string;
	}

	const allVideos: DailyVideo[] = [
		{
			id: 1,
			title: 'Weekend Market Review',
			description: 'Bulls held the gains from last week and in the process now have a "prize worth fighting for" into next week as we all wait for Powell and his purple tie. Sam reviews the core markets and the periphery to see how things are aligned into next week.',
			date: 'December 05, 2025',
			trader: 'Sam',
			traderId: 'sam-shames',
			thumbnail: '/images/traders/sam-shames.jpg'
		},
		{
			id: 2,
			title: 'Santa Is Waiting For Powell',
			description: 'Bruce likes to really think about the upcoming economic calendar, such as next week\'s FED meeting, and how this will affect the market. He also looks at interesting stock ideas, such as LRCX, APH, MTZ, XLK, among others.',
			date: 'December 04, 2025',
			trader: 'Bruce Marshall',
			traderId: 'bruce-marshall',
			thumbnail: '/images/traders/bruce-marshall.jpg'
		},
		{
			id: 3,
			title: 'Ready For The Ripple',
			description: 'It\'s said there can be a ripple effect in equities due to the impact of currencies. Henry thinks the stage is set for that, we just want to monitor for continuation.',
			date: 'December 03, 2025',
			trader: 'Henry Gambell',
			traderId: 'henry-gambell',
			thumbnail: '/images/traders/henry-gambell.jpg'
		},
		{
			id: 4,
			title: 'SPX And The Mag7... What Comes Next?',
			description: 'After big rallies last week, the markets have been digesting gains, with AAPL and GOOGL ripping new highs while META and MSFT wallow in shame. How does this impact the SPX?',
			date: 'December 02, 2025',
			trader: 'John Carter',
			traderId: 'john-carter',
			thumbnail: '/images/traders/john-carter.jpg'
		},
		{
			id: 5,
			title: 'Welcome To December',
			description: 'We saw a fantastic rally in the market last week, but this week has started off with a bit of softness. Overall, the indexes still look good to continue higher. Danielle looks at MSFT, TSLA, NVDA, AVGO, and more.',
			date: 'December 01, 2025',
			trader: 'Danielle Shay',
			traderId: 'danielle-shay',
			thumbnail: '/images/traders/danielle-shay.jpg'
		},
		{
			id: 6,
			title: 'Weekend Market Review',
			description: 'Huge week by bulls to recover some of the damage to the longer term charts, but is it out of the woods yet? Sam reviews the core markets and the periphery to see how things are aligned into next week.',
			date: 'November 28, 2025',
			trader: 'Sam',
			traderId: 'sam-shames',
			thumbnail: '/images/traders/sam-shames.jpg'
		},
		{
			id: 7,
			title: "Correction: Now It's Turkey Time",
			description: "Henry's view from last week was off by about a day and a half, but it came through this week with flying colors. Let's revisit that, and what it might mean for the rest of the year.",
			date: 'November 26, 2025',
			trader: 'Henry Gambell',
			traderId: 'henry-gambell',
			thumbnail: '/images/traders/henry-gambell.jpg'
		},
		{
			id: 8,
			title: 'Market Room Recap: SPX',
			description: 'Allison walks through the recent action SPX has taken the past week, and why she is a cautious bull to the upside.',
			date: 'November 25, 2025',
			trader: 'Allison Ostrander',
			traderId: 'allison-ostrander',
			thumbnail: '/images/traders/allison-ostrander.jpg'
		},
		{
			id: 9,
			title: 'Rallying, Potential Continuation',
			description: 'Markets have shown strength and Danielle is looking for continuation patterns that could lead to extended gains.',
			date: 'November 24, 2025',
			trader: 'Danielle Shay',
			traderId: 'danielle-shay',
			thumbnail: '/images/traders/danielle-shay.jpg'
		},
		{
			id: 10,
			title: 'Thanksgiving Week Trading',
			description: 'Taylor discusses holiday week trading strategies and what to watch for in the shortened trading week.',
			date: 'November 22, 2025',
			trader: 'Taylor Horton',
			traderId: 'taylor-horton',
			thumbnail: '/images/traders/taylor-horton.jpg'
		},
		{
			id: 11,
			title: 'Sector Rotation Opportunities',
			description: 'Bruce examines which sectors are showing relative strength and where traders should be focusing their attention.',
			date: 'November 21, 2025',
			trader: 'Bruce Marshall',
			traderId: 'bruce-marshall',
			thumbnail: '/images/traders/bruce-marshall.jpg'
		},
		{
			id: 12,
			title: 'Options Expiration Analysis',
			description: 'John breaks down the key levels to watch heading into options expiration and potential volatility plays.',
			date: 'November 20, 2025',
			trader: 'John Carter',
			traderId: 'john-carter',
			thumbnail: '/images/traders/john-carter.jpg'
		}
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredVideos = $derived.by(() => {
		return allVideos.filter(video => {
			const matchesTrader = selectedTrader === 'all' || video.traderId === selectedTrader;
			const matchesSearch = searchQuery === '' ||
				video.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				video.description.toLowerCase().includes(searchQuery.toLowerCase()) ||
				video.trader.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesTrader && matchesSearch;
		});
	});

	const totalPages = $derived(Math.ceil(filteredVideos.length / itemsPerPage));

	const paginatedVideos = $derived.by(() => {
		const start = (currentPage - 1) * itemsPerPage;
		return filteredVideos.slice(start, start + itemsPerPage);
	});

	// Reset to page 1 when filters change
	$effect(() => {
		searchQuery;
		selectedTrader;
		currentPage = 1;
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>{membershipTitle} Premium Daily Videos | Dashboard | Revolution Trading Pros</title>
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- ═══════════════════════════════════════════════════════════════════════════
     DASHBOARD HEADER
     ═══════════════════════════════════════════════════════════════════════════ -->

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<nav class="dashboard__breadcrumb" aria-label="Breadcrumb">
			<a href="/dashboard">Dashboard</a>
			<span class="separator">/</span>
			<a href="/dashboard/{slug}">{membershipTitle}</a>
			<span class="separator">/</span>
			<span class="current">Premium Daily Videos</span>
		</nav>
		<h1 class="dashboard__page-title">
			<span class="st-icon-daily-videos"></span>
			Premium Daily Videos
		</h1>
	</div>
	<div class="dashboard__header-right">
		<span class="video-count">Showing {filteredVideos.length} videos</span>
	</div>
</header>

<!-- ═══════════════════════════════════════════════════════════════════════════
     SECONDARY NAVIGATION
     ═══════════════════════════════════════════════════════════════════════════ -->

<nav class="dashboard__nav-secondary">
	<ul class="nav-menu">
		<li class="nav-item">
			<a href="/dashboard/{slug}">
				<span class="st-icon-dashboard nav-icon"></span>
				<span class="nav-text">{membershipTitle} Dashboard</span>
			</a>
		</li>
		<li class="nav-item is-active">
			<a href="/dashboard/{slug}/daily-videos">
				<span class="st-icon-daily-videos nav-icon"></span>
				<span class="nav-text">Premium Daily Videos</span>
			</a>
		</li>
		<li class="nav-item">
			<a href="/dashboard/{slug}/learning-center">
				<span class="st-icon-learning-center nav-icon"></span>
				<span class="nav-text">Learning Center</span>
			</a>
		</li>
		<li class="nav-item">
			<a href="/dashboard/{slug}/archive">
				<span class="st-icon-chatroom-archive nav-icon"></span>
				<span class="nav-text">Trading Room Archives</span>
			</a>
		</li>
	</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Section Title -->
		<section class="dashboard__content-section">
			<h2 class="section-title">{membershipTitle} Premium Daily Videos</h2>

			<!-- Filters Bar -->
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					<span class="facetwp-counts">{filteredVideos.length} of {allVideos.length}</span>
				</div>
				<div class="dashboard-filters__search">
					<div class="search-box">
						<IconSearch size={18} />
						<input
							id="video-search"
							type="text"
							placeholder="Search videos..."
							bind:value={searchQuery}
						/>
					</div>
				</div>
			</div>

			<!-- Trader Filter -->
			<div class="trader-filter">
				<IconFilter size={18} />
				<select bind:value={selectedTrader}>
					{#each traders as trader}
						<option value={trader.id}>{trader.name}</option>
					{/each}
				</select>
			</div>

			<!-- Videos Grid - Matches SimplerPremiumDailyVideos structure -->
			{#if paginatedVideos.length > 0}
				<div class="card-grid flex-grid row">
					{#each paginatedVideos as video (video.id)}
						<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-lg-4">
							<div class="card flex-grid-panel">
								<figure class="card-media card-media--video">
									<a href="/dashboard/{slug}/daily-videos/{video.id}" class="card-image" style:background-image="url({video.thumbnail})">
										<span class="card-play-icon">
											<IconPlayerPlay size={32} />
										</span>
									</a>
								</figure>
								<section class="card-body">
									<h4 class="h5 card-title">
										<a href="/dashboard/{slug}/daily-videos/{video.id}">
											{video.title}
										</a>
									</h4>
									<span class="article-card__meta">
										<small>{video.date} with {video.trader}</small>
									</span>
									<div class="card-description">
										<div class="u--hide-read-more u--squash">
											<p>{video.description}</p>
										</div>
									</div>
								</section>
								<footer class="card-footer">
									<a class="btn btn-tiny btn-default" href="/dashboard/{slug}/daily-videos/{video.id}">Watch Now</a>
								</footer>
							</div>
						</article>
					{/each}
				</div>

				<!-- Pagination -->
				{#if totalPages > 1}
					<nav class="pagination" aria-label="Daily videos pagination">
						<button
							class="pagination-btn"
							disabled={currentPage === 1}
							onclick={() => currentPage--}
						>
							<IconChevronLeft size={18} />
							Previous
						</button>
						<div class="pagination-info">
							Page {currentPage} of {totalPages}
						</div>
						<button
							class="pagination-btn"
							disabled={currentPage === totalPages}
							onclick={() => currentPage++}
						>
							Next
							<IconChevronRight size={18} />
						</button>
					</nav>
				{/if}
			{:else}
				<div class="empty-state">
					<IconVideo size={64} />
					<h3>No videos found</h3>
					<p>Try adjusting your search or filter criteria</p>
				</div>
			{/if}
		</section>
	</div>
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES - WordPress Revolution Trading Exact Match
     ═══════════════════════════════════════════════════════════════════════════ -->

<style>
	/* Dashboard Header */
	.dashboard__header {
		background-color: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 20px 30px;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		align-items: center;
		gap: 16px;
	}

	.dashboard__header-left {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}

	.dashboard__breadcrumb {
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	.dashboard__breadcrumb a {
		color: var(--st-link-color, #1e73be);
		text-decoration: none;
	}

	.dashboard__breadcrumb a:hover {
		text-decoration: underline;
	}

	.dashboard__breadcrumb .separator {
		margin: 0 8px;
		color: #999;
	}

	.dashboard__page-title {
		display: flex;
		align-items: center;
		gap: 12px;
		color: var(--st-text-color, #333);
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 32px;
		font-weight: 700;
		margin: 0;
	}

	.dashboard__page-title .st-icon-daily-videos {
		font-size: 32px;
		color: var(--st-primary, #0984ae);
	}

	.video-count {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
		background: #f1f5f9;
		padding: 8px 16px;
		border-radius: 20px;
	}

	/* Secondary Navigation */
	.dashboard__nav-secondary {
		background: #fff;
		border-bottom: 1px solid var(--st-border-color, #dbdbdb);
		padding: 0 30px;
	}

	.nav-menu {
		display: flex;
		list-style: none;
		margin: 0;
		padding: 0;
		overflow-x: auto;
	}

	.nav-item a {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 16px 20px;
		color: var(--st-text-muted, #64748b);
		text-decoration: none;
		font-size: 14px;
		font-weight: 500;
		border-bottom: 3px solid transparent;
		transition: all 0.15s ease;
		white-space: nowrap;
	}

	.nav-item a:hover {
		color: var(--st-primary, #0984ae);
	}

	.nav-item.is-active a {
		color: var(--st-primary, #0984ae);
		border-bottom-color: var(--st-primary, #0984ae);
	}

	.nav-icon {
		font-size: 18px;
	}

	/* Content */
	.dashboard__content {
		padding: 30px;
		background: #fff;
		min-height: calc(100vh - 200px);
	}

	.dashboard__content-main {
		max-width: 1200px;
	}

	.section-title {
		font-family: 'Open Sans Condensed', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: var(--st-text-color, #333);
		margin: 0 0 24px;
	}

	/* Filters - WordPress FacetWP Style */
	.dashboard-filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 16px;
		flex-wrap: wrap;
		gap: 16px;
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
	}

	.dashboard-filters__search {
		flex: 1;
		max-width: 350px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		color: var(--st-text-muted, #64748b);
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
	}

	.trader-filter {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		color: var(--st-text-muted, #64748b);
		margin-bottom: 24px;
	}

	.trader-filter select {
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
		cursor: pointer;
		color: var(--st-text-color, #333);
	}

	/* Card Grid - WordPress Flex Grid Exact Match */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -12px;
	}

	.card-grid-spacer {
		padding: 0 12px;
		margin-bottom: 24px;
	}

	.col-xs-12 {
		width: 100%;
	}

	.col-sm-6 {
		width: 50%;
	}

	.col-lg-4 {
		width: 33.333%;
	}

	/* Card - WordPress Style */
	.card {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: #fff;
		border: 1px solid #ededed;
		border-radius: 8px;
		overflow: hidden;
		transition: all 0.2s ease;
	}

	.card:hover {
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
		transform: translateY(-2px);
	}

	.card-media {
		position: relative;
		margin: 0;
	}

	.card-media--video {
		height: 183px;
	}

	.card-image {
		display: block;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		background-color: #1e293b;
	}

	.card-play-icon {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 56px;
		height: 56px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: rgba(255, 255, 255, 0.9);
		border-radius: 50%;
		color: var(--st-primary, #0984ae);
		opacity: 0;
		transition: opacity 0.2s ease;
	}

	.card:hover .card-play-icon {
		opacity: 1;
	}

	.card-body {
		flex: 1;
		padding: 16px;
	}

	.h5.card-title {
		font-size: 16px;
		font-weight: 600;
		line-height: 1.4;
		margin: 0 0 8px;
	}

	.card-title a {
		color: var(--st-text-color, #333);
		text-decoration: none;
	}

	.card-title a:hover {
		color: var(--st-primary, #0984ae);
	}

	.article-card__meta {
		display: block;
		margin-bottom: 12px;
	}

	.article-card__meta small {
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	.card-description {
		color: var(--st-text-muted, #64748b);
		font-size: 14px;
		line-height: 1.5;
	}

	.card-description p {
		margin: 0;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.card-footer {
		padding: 0 16px 16px;
	}

	/* Buttons - WordPress Style */
	.btn {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease;
		border: none;
	}

	.btn-tiny {
		padding: 8px 14px;
		font-size: 13px;
	}

	.btn-default {
		background: #f4f4f4;
		border: 1px solid #dbdbdb;
		color: var(--st-text-color, #333);
	}

	.btn-default:hover {
		background: #e9e9e9;
		border-color: #c1c1c1;
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 16px;
		padding: 32px 0;
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		background: #fff;
		border: 1px solid #dbdbdb;
		border-radius: 4px;
		font-size: 14px;
		font-weight: 500;
		color: var(--st-text-color, #333);
		cursor: pointer;
		transition: all 0.15s ease;
	}

	.pagination-btn:hover:not(:disabled) {
		border-color: var(--st-primary, #0984ae);
		color: var(--st-primary, #0984ae);
	}

	.pagination-btn:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	.pagination-info {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 80px 20px;
		color: var(--st-text-muted, #64748b);
	}

	.empty-state h3 {
		margin: 20px 0 8px;
		font-size: 20px;
		color: var(--st-text-color, #333);
	}

	.empty-state p {
		margin: 0;
	}

	/* Responsive */
	@media (max-width: 991px) {
		.col-lg-4 {
			width: 50%;
		}
	}

	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__nav-secondary {
			padding: 0 16px;
		}

		.dashboard__content {
			padding: 16px;
		}

		.dashboard-filters {
			flex-direction: column;
			align-items: stretch;
		}

		.dashboard-filters__search {
			max-width: none;
		}

		.col-sm-6, .col-lg-4 {
			width: 100%;
		}

		.card-grid-spacer {
			padding: 0;
		}

		.pagination {
			flex-wrap: wrap;
		}
	}
</style>
