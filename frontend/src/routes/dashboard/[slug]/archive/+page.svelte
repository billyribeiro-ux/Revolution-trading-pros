<script lang="ts">
	/**
	 * Video Archive Page - WordPress Revolution Trading Exact
	 * ═══════════════════════════════════════════════════════════════════════════
	 *
	 * URL: /dashboard/mastering-the-trade/archive
	 * Shows archived trading room recordings with search and filter capabilities.
	 *
	 * @version 1.0.0 (December 2025)
	 */

	import { page } from '$app/stores';
	import IconVideo from '@tabler/icons-svelte/icons/video';
	import IconPlayerPlay from '@tabler/icons-svelte/icons/player-play';
	import IconCalendar from '@tabler/icons-svelte/icons/calendar';
	import IconClock from '@tabler/icons-svelte/icons/clock';
	import IconSearch from '@tabler/icons-svelte/icons/search';
	import IconFilter from '@tabler/icons-svelte/icons/filter';
	import IconChevronLeft from '@tabler/icons-svelte/icons/chevron-left';
	import IconChevronRight from '@tabler/icons-svelte/icons/chevron-right';
	import IconDownload from '@tabler/icons-svelte/icons/download';
	import '$lib/styles/st-icons.css';

	// ═══════════════════════════════════════════════════════════════════════════
	// ROUTE PARAMS
	// ═══════════════════════════════════════════════════════════════════════════

	const slug = $derived($page.params.slug);

	// ═══════════════════════════════════════════════════════════════════════════
	// STATE
	// ═══════════════════════════════════════════════════════════════════════════

	let searchQuery = $state('');
	let selectedMonth = $state('all');
	let currentPage = $state(1);
	const itemsPerPage = 12;

	// ═══════════════════════════════════════════════════════════════════════════
	// MONTHS FILTER
	// ═══════════════════════════════════════════════════════════════════════════

	const months = [
		{ id: 'all', name: 'All Time' },
		{ id: '2025-12', name: 'December 2025' },
		{ id: '2025-11', name: 'November 2025' },
		{ id: '2025-10', name: 'October 2025' },
		{ id: '2025-09', name: 'September 2025' },
		{ id: '2025-08', name: 'August 2025' }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// VIDEO ARCHIVE (Mock data)
	// ═══════════════════════════════════════════════════════════════════════════

	interface VideoItem {
		id: number;
		title: string;
		description: string;
		duration: string;
		date: string;
		month: string;
		views: number;
		thumbnail?: string;
	}

	const allVideos: VideoItem[] = [
		{ id: 1, title: 'Morning Market Analysis - SPY Setup', description: 'Live analysis of SPY chart patterns and potential trade setups for the day.', duration: '1:23:45', date: 'Dec 6, 2025', month: '2025-12', views: 234 },
		{ id: 2, title: 'Options Trade Breakdown - NVDA Calls', description: 'Detailed breakdown of the NVDA options trade from the morning session.', duration: '45:30', date: 'Dec 5, 2025', month: '2025-12', views: 189 },
		{ id: 3, title: 'Weekly Recap & Planning Session', description: 'Review of the week trades and planning for next week opportunities.', duration: '58:12', date: 'Dec 4, 2025', month: '2025-12', views: 312 },
		{ id: 4, title: 'AAPL Earnings Play Strategy', description: 'Pre-earnings strategy discussion for Apple quarterly report.', duration: '42:18', date: 'Dec 3, 2025', month: '2025-12', views: 267 },
		{ id: 5, title: 'Market Open Scalping Techniques', description: 'Fast-paced scalping strategies for the first 30 minutes of market open.', duration: '1:15:22', date: 'Dec 2, 2025', month: '2025-12', views: 198 },
		{ id: 6, title: 'Swing Trade Setups Review', description: 'Multi-day swing trade opportunities in tech and energy sectors.', duration: '55:40', date: 'Nov 29, 2025', month: '2025-11', views: 276 },
		{ id: 7, title: 'Risk Management Deep Dive', description: 'Comprehensive session on position sizing and risk control.', duration: '1:08:15', date: 'Nov 28, 2025', month: '2025-11', views: 423 },
		{ id: 8, title: 'Technical Analysis Workshop', description: 'Support and resistance levels, trend lines, and key indicators.', duration: '1:32:00', date: 'Nov 27, 2025', month: '2025-11', views: 387 },
		{ id: 9, title: 'Fed Meeting Analysis', description: 'Live reaction and trading during Federal Reserve announcement.', duration: '2:15:30', date: 'Nov 26, 2025', month: '2025-11', views: 521 },
		{ id: 10, title: 'Holiday Trading Strategies', description: 'Low volume trading tactics for holiday-shortened weeks.', duration: '48:22', date: 'Nov 25, 2025', month: '2025-11', views: 156 },
		{ id: 11, title: 'Sector Rotation Analysis', description: 'Identifying sector strength and weakness for trade selection.', duration: '1:05:18', date: 'Oct 31, 2025', month: '2025-10', views: 298 },
		{ id: 12, title: 'Options Greeks Explained', description: 'Understanding Delta, Gamma, Theta, and Vega for better trades.', duration: '1:28:45', date: 'Oct 30, 2025', month: '2025-10', views: 445 },
		{ id: 13, title: 'Volatility Trading Strategies', description: 'How to profit from high and low volatility environments.', duration: '1:12:33', date: 'Oct 29, 2025', month: '2025-10', views: 334 },
		{ id: 14, title: 'Chart Pattern Recognition', description: 'Identifying and trading classic chart patterns.', duration: '59:48', date: 'Oct 28, 2025', month: '2025-10', views: 412 },
		{ id: 15, title: 'Pre-Market Preparation Routine', description: 'How to prepare for the trading day before market open.', duration: '35:20', date: 'Oct 25, 2025', month: '2025-10', views: 287 }
	];

	// ═══════════════════════════════════════════════════════════════════════════
	// DERIVED STATE
	// ═══════════════════════════════════════════════════════════════════════════

	const filteredVideos = $derived.by(() => {
		return allVideos.filter(video => {
			const matchesMonth = selectedMonth === 'all' || video.month === selectedMonth;
			const matchesSearch = searchQuery === '' ||
				video.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				video.description.toLowerCase().includes(searchQuery.toLowerCase());
			return matchesMonth && matchesSearch;
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
		selectedMonth;
		currentPage = 1;
	});
</script>

<!-- ═══════════════════════════════════════════════════════════════════════════
     HEAD
     ═══════════════════════════════════════════════════════════════════════════ -->

<svelte:head>
	<title>Video Archive | Dashboard | Revolution Trading Pros</title>
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
			<a href="/dashboard/{slug}">{slug.split('-').map(w => w.charAt(0).toUpperCase() + w.slice(1)).join(' ')}</a>
			<span class="separator">/</span>
			<span class="current">Video Archive</span>
		</nav>
		<h1 class="dashboard__page-title">
			<span class="st-icon-chatroom-archive"></span>
			Video Archive
		</h1>
	</div>
	<div class="dashboard__header-right">
		<span class="video-count">{filteredVideos.length} videos</span>
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
				<span class="nav-text">Dashboard</span>
			</a>
		</li>
		<li class="nav-item">
			<a href="/dashboard/{slug}/learning-center">
				<span class="st-icon-learning-center nav-icon"></span>
				<span class="nav-text">Learning Center</span>
			</a>
		</li>
		<li class="nav-item is-active">
			<a href="/dashboard/{slug}/archive">
				<span class="st-icon-chatroom-archive nav-icon"></span>
				<span class="nav-text">Video Archive</span>
			</a>
		</li>
	</ul>
</nav>

<!-- ═══════════════════════════════════════════════════════════════════════════
     CONTENT
     ═══════════════════════════════════════════════════════════════════════════ -->

<div class="dashboard__content">
	<!-- Filters Bar -->
	<div class="filters-bar">
		<div class="search-box">
			<IconSearch size={18} />
			<input
				id="video-search"
				type="text"
				placeholder="Search videos..."
				bind:value={searchQuery}
			/>
		</div>
		<div class="month-filter">
			<IconFilter size={18} />
			<select bind:value={selectedMonth}>
				{#each months as month}
					<option value={month.id}>{month.name}</option>
				{/each}
			</select>
		</div>
	</div>

	<!-- Videos Grid -->
	{#if paginatedVideos.length > 0}
		<div class="videos-grid">
			{#each paginatedVideos as video (video.id)}
				<a href="/dashboard/{slug}/archive/{video.id}" class="video-card">
					<div class="video-card__thumbnail">
						<div class="video-card__placeholder">
							<IconVideo size={48} />
						</div>
						<span class="video-card__duration">{video.duration}</span>
						<div class="video-card__play">
							<IconPlayerPlay size={32} />
						</div>
					</div>
					<div class="video-card__content">
						<h3 class="video-card__title">{video.title}</h3>
						<p class="video-card__description">{video.description}</p>
						<div class="video-card__meta">
							<span class="meta-item">
								<IconCalendar size={14} />
								{video.date}
							</span>
							<span class="meta-item">
								<IconPlayerPlay size={14} />
								{video.views} views
							</span>
						</div>
					</div>
				</a>
			{/each}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<nav class="pagination" aria-label="Video archive pagination">
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
</div>

<!-- ═══════════════════════════════════════════════════════════════════════════
     STYLES
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

	.dashboard__page-title .st-icon-chatroom-archive {
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
		background: #f8f9fa;
		min-height: calc(100vh - 200px);
	}

	/* Filters Bar */
	.filters-bar {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
		margin-bottom: 24px;
	}

	.search-box {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		flex: 1;
		max-width: 400px;
		color: var(--st-text-muted, #64748b);
	}

	.search-box input {
		flex: 1;
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
	}

	.month-filter {
		display: flex;
		align-items: center;
		gap: 8px;
		padding: 10px 16px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
		color: var(--st-text-muted, #64748b);
	}

	.month-filter select {
		border: none;
		outline: none;
		font-size: 14px;
		background: transparent;
		cursor: pointer;
		color: var(--st-text-color, #333);
	}

	/* Videos Grid */
	.videos-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(min(100%, 280px), 1fr));
		gap: 20px;
		margin-bottom: 32px;
	}

	.video-card {
		display: block;
		background: #fff;
		border-radius: 12px;
		overflow: hidden;
		text-decoration: none;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.08);
		transition: all 0.2s ease;
	}

	.video-card:hover {
		transform: translateY(-4px);
		box-shadow: 0 8px 24px rgba(0, 0, 0, 0.12);
	}

	.video-card:hover .video-card__play {
		opacity: 1;
		transform: translate(-50%, -50%) scale(1);
	}

	.video-card__thumbnail {
		position: relative;
		height: 180px;
		background: linear-gradient(135deg, #1e293b, #334155);
	}

	.video-card__placeholder {
		display: flex;
		align-items: center;
		justify-content: center;
		height: 100%;
		color: #475569;
	}

	.video-card__duration {
		position: absolute;
		bottom: 12px;
		right: 12px;
		padding: 4px 10px;
		background: rgba(0, 0, 0, 0.85);
		border-radius: 4px;
		font-size: 13px;
		font-weight: 600;
		color: #fff;
	}

	.video-card__play {
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%) scale(0.9);
		width: 64px;
		height: 64px;
		display: flex;
		align-items: center;
		justify-content: center;
		background: var(--st-primary, #0984ae);
		border-radius: 50%;
		color: #fff;
		opacity: 0;
		transition: all 0.2s ease;
	}

	.video-card__content {
		padding: 20px;
	}

	.video-card__title {
		font-size: 16px;
		font-weight: 600;
		color: var(--st-text-color, #333);
		margin: 0 0 8px;
		line-height: 1.4;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-card__description {
		font-size: 14px;
		color: var(--st-text-muted, #64748b);
		margin: 0 0 12px;
		line-height: 1.5;
		display: -webkit-box;
		-webkit-line-clamp: 2;
		line-clamp: 2;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	.video-card__meta {
		display: flex;
		gap: 16px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 4px;
		font-size: 13px;
		color: var(--st-text-muted, #64748b);
	}

	/* Pagination */
	.pagination {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 16px;
		padding: 20px 0;
	}

	.pagination-btn {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 10px 20px;
		background: #fff;
		border: 1px solid #e5e7eb;
		border-radius: 8px;
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
	@media (max-width: 768px) {
		.dashboard__header {
			padding: 16px;
		}

		.dashboard__page-title {
			font-size: 24px;
		}

		.dashboard__nav-secondary {
			padding: 0 16px;
			overflow-x: auto;
		}

		.dashboard__content {
			padding: 16px;
		}

		.filters-bar {
			flex-direction: column;
		}

		.search-box {
			max-width: none;
		}

		.videos-grid {
			grid-template-columns: 1fr;
		}

		.pagination {
			flex-wrap: wrap;
		}
	}
</style>
