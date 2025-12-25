<script lang="ts">
	/**
	 * Weekly Watchlist Archive - 100% PIXEL-PERFECT Match
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Archive of past weekly watchlists
	 * @version 1.0.0 - Pixel Perfect
	 */

	// Archive data
	const archives = [
		{ id: 1, week: 'December 16-20, 2025', trader: 'TG Watkins', stockCount: 5, date: 'December 15, 2025', slug: 'dec-16-20-2025' },
		{ id: 2, week: 'December 9-13, 2025', trader: 'TG Watkins', stockCount: 6, date: 'December 8, 2025', slug: 'dec-9-13-2025' },
		{ id: 3, week: 'December 2-6, 2025', trader: 'TG Watkins', stockCount: 5, date: 'December 1, 2025', slug: 'dec-2-6-2025' },
		{ id: 4, week: 'November 25-29, 2025', trader: 'TG Watkins', stockCount: 4, date: 'November 24, 2025', slug: 'nov-25-29-2025' },
		{ id: 5, week: 'November 18-22, 2025', trader: 'TG Watkins', stockCount: 5, date: 'November 17, 2025', slug: 'nov-18-22-2025' },
		{ id: 6, week: 'November 11-15, 2025', trader: 'TG Watkins', stockCount: 6, date: 'November 10, 2025', slug: 'nov-11-15-2025' },
		{ id: 7, week: 'November 4-8, 2025', trader: 'TG Watkins', stockCount: 5, date: 'November 3, 2025', slug: 'nov-4-8-2025' },
		{ id: 8, week: 'October 28 - November 1, 2025', trader: 'TG Watkins', stockCount: 5, date: 'October 27, 2025', slug: 'oct-28-nov-1-2025' }
	];

	// Pagination
	let currentPage = $state(1);
	const itemsPerPage = 10;
</script>

<svelte:head>
	<title>Weekly Watchlist Archive | Revolution Trading Pros</title>
	<meta name="description" content="Archive of past weekly trading watchlists with historical stock picks and analysis." />
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
				<li class="item-current"><strong class="breadcrumb-current">Archive</strong></li>
			</ul>
		</div>
	</nav>

	<!-- Page Content -->
	<div id="page" class="hfeed site grid-parent">
		<div id="content" class="site-content">

			<!-- Page Header -->
			<header class="page-header-section">
				<div class="container-fluid">
					<h1 class="page-title">Weekly Watchlist Archive</h1>
					<p class="page-subtitle">Browse past weekly watchlists and historical stock picks</p>
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

					<!-- Archive Grid -->
					<div class="archive-grid">
						{#each archives as archive}
							<article class="archive-card">
								<div class="archive-card-header">
									<span class="archive-icon">
										<svg viewBox="0 0 24 24" width="24" height="24">
											<path fill="currentColor" d="M19 3H5c-1.1 0-2 .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm0 16H5V5h14v14zm-7-2h2V7h-4v2h2z"/>
										</svg>
									</span>
									<span class="archive-date">{archive.date}</span>
								</div>
								<div class="archive-card-body">
									<h3 class="archive-title">Week of {archive.week}</h3>
									<div class="archive-meta">
										<span class="meta-item">
											<svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm0 18c-4.41 0-8-3.59-8-8s3.59-8 8-8 8 3.59 8 8-3.59 8-8 8zm-1-13h2v6h-2zm0 8h2v2h-2z"/></svg>
											{archive.trader}
										</span>
										<span class="meta-item">
											<svg viewBox="0 0 24 24" width="14" height="14"><path fill="currentColor" d="M3 13h2v-2H3v2zm0 4h2v-2H3v2zm0-8h2V7H3v2zm4 4h14v-2H7v2zm0 4h14v-2H7v2zM7 7v2h14V7H7z"/></svg>
											{archive.stockCount} Stocks
										</span>
									</div>
								</div>
								<div class="archive-card-footer">
									<a href="/weeklywatchlist/archive/{archive.slug}" class="view-btn">View Watchlist</a>
								</div>
							</article>
						{/each}
					</div>

					<!-- Pagination -->
					<div class="pagination">
						<span class="page-info">Showing 1-8 of 52 watchlists</span>
						<div class="page-controls">
							<button class="page-btn" disabled>Previous</button>
							<button class="page-btn page-btn-active">1</button>
							<button class="page-btn">2</button>
							<button class="page-btn">3</button>
							<span class="page-dots">...</span>
							<button class="page-btn">7</button>
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

	/* Archive Grid */
	.archive-grid {
		display: grid;
		grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
		gap: 20px;
		margin-bottom: 40px;
	}

	.archive-card {
		background: #fff;
		border-radius: 8px;
		overflow: hidden;
		box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.archive-card:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 16px rgba(0, 0, 0, 0.12);
	}

	.archive-card-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		padding: 16px 20px;
		background: #f9f9f9;
		border-bottom: 1px solid #ededed;
	}

	.archive-icon {
		display: flex;
		align-items: center;
		justify-content: center;
		width: 40px;
		height: 40px;
		background: #0984ae;
		border-radius: 8px;
		color: #fff;
	}

	.archive-date {
		font-size: 13px;
		color: #666;
		font-weight: 500;
	}

	.archive-card-body {
		padding: 20px;
	}

	.archive-title {
		font-size: 16px;
		font-weight: 700;
		color: #1a1a1a;
		margin: 0 0 12px;
	}

	.archive-meta {
		display: flex;
		flex-wrap: wrap;
		gap: 16px;
	}

	.meta-item {
		display: flex;
		align-items: center;
		gap: 6px;
		font-size: 13px;
		color: #666;
	}

	.archive-card-footer {
		padding: 16px 20px;
		border-top: 1px solid #ededed;
	}

	.view-btn {
		display: inline-block;
		padding: 10px 20px;
		background: #0984ae;
		color: #fff;
		text-decoration: none;
		border-radius: 5px;
		font-size: 14px;
		font-weight: 600;
		transition: background 0.15s ease;
	}

	.view-btn:hover {
		background: #076787;
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

		.archive-grid {
			grid-template-columns: 1fr;
		}

		.pagination {
			flex-direction: column;
			align-items: flex-start;
		}
	}
</style>
