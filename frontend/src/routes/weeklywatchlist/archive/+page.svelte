<script lang="ts">
	/**
	 * Weekly Watchlist Archive - Matching Simpler Trading
	 * ═══════════════════════════════════════════════════════════════════════════
	 * Archive of past weekly watchlists (spreadsheet tab)
	 * @version 2.0.0 - December 2025
	 */

	// Archive data - matches reference URL structure
	const archives = [
		{ date: 'December 22, 2025', trader: 'TG Watkins', slug: '12222025-tg-watkins' },
		{ date: 'December 15, 2025', trader: 'Allison Ostrander', slug: '12152025-allison-ostrander' },
		{ date: 'December 8, 2025', trader: 'Taylor Horton', slug: '12082025-taylor-horton' },
		{ date: 'December 1, 2025', trader: 'Raghee Horner', slug: '12012025-raghee-horner' },
		{ date: 'November 24, 2025', trader: 'David Starr', slug: '11242025-david-starr' },
		{ date: 'November 17, 2025', trader: 'Mike Teeto', slug: '11172025-mike-teeto' },
		{ date: 'November 10, 2025', trader: 'Taylor Horton', slug: '11102025-taylor-horton' },
		{ date: 'November 3, 2025', trader: 'Allison Ostrander', slug: '11032025-allison-ostrander' },
		{ date: 'October 27, 2025', trader: 'Bruce Marshall', slug: '10272025-bruce-marshall' },
		{ date: 'October 20, 2025', trader: 'Jonathan McKeever', slug: 'tr3ndy-10202025' },
		{ date: 'October 12, 2025', trader: 'TG Watkins', slug: '10122025-tg-watkins' },
		{ date: 'October 6, 2025', trader: 'Henry Gambell', slug: '10062025-henry-gambell' }
	];

	// Pagination state
	let currentPage = $state(1);
	const totalPages = 22;
</script>

<svelte:head>
	<title>Weekly Watchlist Archive - Revolution Trading Pros</title>
	<meta name="description" content="Archive of past weekly trading watchlists" />
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
				<li>
					<a href="/dashboard/ww/watchlist-rundown-archive/">
						<span class="dashboard__nav-item-icon st-icon-chatroom-archive"></span>
						<span class="dashboard__nav-item-text">Watchlist Rundown Archive</span>
					</a>
				</li>
				<li class="is-active">
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
				<section class="dashboard__content-section">
					<div class="fl-post-grid-post">
						{#each archives as item}
							<div class="card fl-post-text">
								<div class="">
									<section class="card-body u--squash">
										<h4 class="h5 card-title">Weekly Watchlist for {item.date}</h4>
										<div class="excerpt"><i>With {item.trader}</i></div>
										<div class="fl-post-more-link">
											<a class="btn btn-tiny btn-default" href="/watchlist/{item.slug}?tab=2">Read Now</a>
										</div>
									</section>
								</div>
							</div>
						{/each}
					</div>

					<!-- Pagination -->
					<div class="text-center pagination-container">
						<div id="loopage_pg" class="pagination-wrap">
							{#if currentPage > 1}
								<a class="page-numbers" href="?pg={currentPage - 1}">&laquo;</a>
							{/if}

							<span aria-current="page" class="page-numbers current">{currentPage}</span>

							{#if currentPage < totalPages}
								<a class="page-numbers" href="?pg={currentPage + 1}">{currentPage + 1}</a>
							{/if}

							{#if currentPage + 1 < totalPages}
								<a class="page-numbers" href="?pg={currentPage + 2}">{currentPage + 2}</a>
							{/if}

							{#if currentPage + 2 < totalPages}
								<span class="page-numbers dots">&hellip;</span>
								<a class="page-numbers" href="?pg={totalPages}">{totalPages}</a>
							{/if}

							{#if currentPage < totalPages}
								<a class="next page-numbers" href="?pg={currentPage + 1}">&raquo;</a>
							{/if}
						</div>
					</div>
				</section>
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
	   Weekly Watchlist Archive - Matching Simpler Trading Exactly
	   ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Layout */
	.dashboard {
		display: flex;
		min-height: 100vh;
		background: #f4f4f4;
	}

	/* Sidebar - ICT 11 canonical: 280px width, #0e2433 background */
	.dashboard__sidebar {
		width: 280px;
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

	/* Archive Grid - Matching Reference */
	.fl-post-grid-post {
		font-size: 19px;
		background: transparent !important;
		border: 0;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		gap: 1%;
	}

	.card {
		max-width: 32%;
		margin: 0.5%;
		background: #fff;
		border-radius: 4px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
		flex: 1 1 calc(33.333% - 1%);
		min-width: 280px;
	}

	.card-body.u--squash {
		padding: 20px;
	}

	.card-title {
		font-size: 24px;
		line-height: 1.3;
		margin: 0 0 10px;
		color: #333;
		font-weight: 600;
	}

	.excerpt {
		margin: 20px 0;
		font-size: 16px;
		color: #666;
	}

	.fl-post-more-link {
		margin-top: 15px;
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

	/* Pagination - Matching Reference */
	.pagination-container {
		margin-top: 20px;
		text-align: center;
	}

	.pagination-wrap {
		display: inline-flex;
		gap: 0;
	}

	.page-numbers {
		display: inline-block;
		padding: 8px 14px;
		font-size: 17px;
		font-weight: 600;
		color: #666;
		background: #fff !important;
		text-decoration: none;
		border: 1px solid #e6e6e6;
		margin-left: -1px;
	}

	.page-numbers:first-child {
		border-radius: 5px 0 0 5px;
		margin-left: 0;
	}

	.page-numbers:last-child {
		border-radius: 0 5px 5px 0;
	}

	.page-numbers:hover {
		background: #f5f5f5 !important;
	}

	.page-numbers.current {
		color: #f4f4f4;
		background: #0984ae !important;
		border-color: #0984ae;
	}

	.page-numbers.dots {
		border: none;
		background: transparent !important;
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

	.col-sm-6 {
		flex: 0 0 50%;
		max-width: 50%;
		padding: 0 15px;
	}

	.col-lg-5 {
		flex: 0 0 41.666667%;
		max-width: 41.666667%;
	}

	.col-lg-7 {
		flex: 0 0 58.333333%;
		max-width: 58.333333%;
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
		.card {
			flex: 1 1 100%;
			max-width: 100%;
		}

		.col-sm-6 {
			flex: 0 0 100%;
			max-width: 100%;
		}

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
