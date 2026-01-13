<!--
	URL: /dashboard/weekly-watchlist/archive
	
	Weekly Watchlist Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress reference
	
	Features:
	- List layout with border separators
	- Pagination matching WordPress
	- Responsive design
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	// Svelte 5 $props destructuring
	const { data } = $props<{
		data: {
			archiveEntries: Array<{
				slug: string;
				title: string;
				trader: string;
				date: string;
			}>;
			latestWatchlist?: {
				slug: string;
				trader: string;
				weekText: string;
				image: string;
			};
			totalPages: number;
			currentPage: number;
		};
	}>();
	
	// Svelte 5 $derived for computed values
	const hasMorePages = $derived(data.currentPage < data.totalPages);
	const latestWatchlist = $derived(data.latestWatchlist);
</script>

<svelte:head>
	<title>Weekly Watchlist Archive - Revolution Trading Pros</title>
</svelte:head>

<header class="dashboard__header">
	<h1 class="dashboard__page-title">Weekly Watchlist Dashboard</h1>
</header>

<!-- Archive Section -->
<section class="archive-section">
	<div class="archive-list">
		{#each data.archiveEntries as { slug, title, trader }}
			<article class="archive-item">
				<h4 class="archive-item__title">{title}</h4>
				<p class="archive-item__trader"><em>With {trader}</em></p>
				<a class="archive-item__btn" href="/watchlist/{slug}?tab=2">Read Now</a>
			</article>
		{/each}
	</div>

	<!-- Pagination -->
	{#if data.totalPages > 1}
		<div class="pagination">
			<span aria-current="page" class="pagination__number pagination__number--active">{data.currentPage}</span>
			{#if hasMorePages}
				<a class="pagination__number" href="/dashboard/weekly-watchlist/archive?pg=2">2</a>
				<a class="pagination__number" href="/dashboard/weekly-watchlist/archive?pg=3">3</a>
				<span class="pagination__dots">&hellip;</span>
				<a class="pagination__number" href="/dashboard/weekly-watchlist/archive?pg={data.totalPages}">{data.totalPages}</a>
				<a class="pagination__next" href="/dashboard/weekly-watchlist/archive?pg=2">&raquo;</a>
			{/if}
		</div>
	{/if}
</section>

<!-- Weekly Watchlist Featured Section - WordPress Match -->
{#if latestWatchlist}
	<div class="dashboard__content-section u--background-color-white">
		<section>
			<div class="row">
				<div class="col-sm-6 col-lg-5">
					<h2 class="section-title-alt section-title-alt--underline">Weekly Watchlist</h2>
					<div class="hidden-md d-lg-none pb-2">
						<a href="/watchlist/{latestWatchlist.slug}">
							<img src={latestWatchlist.image} alt="Weekly Watchlist" class="u--border-radius" />
						</a>
					</div>
					<h4 class="h5 u--font-weight-bold">Weekly Watchlist with {latestWatchlist.trader}</h4>
					<div class="u--hide-read-more">
						<p>{latestWatchlist.weekText}</p>
					</div>
					<a href="/watchlist/{latestWatchlist.slug}" class="btn btn-tiny btn-default">Watch Now</a>
				</div>
				<div class="col-sm-6 col-lg-7 hidden-xs hidden-sm d-none d-lg-block">
					<a href="/watchlist/{latestWatchlist.slug}">
						<img src={latestWatchlist.image} alt="Weekly Watchlist" class="u--border-radius" />
					</a>
				</div>
			</div>
		</section>
	</div>
{/if}

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

	/* Archive Section */
	.archive-section {
		padding: 20px 30px;
		background: #f5f5f5;
	}

	/* Archive List */
	.archive-list {
		background: #fff;
		border: 1px solid #e0e0e0;
		border-radius: 4px;
	}

	/* Archive Item */
	.archive-item {
		padding: 20px 25px;
		border-bottom: 1px solid #e0e0e0;
	}

	.archive-item:last-child {
		border-bottom: none;
	}

	.archive-item__title {
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 0;
		line-height: 26px;
		padding: 0 0 5px;
	}

	.archive-item__trader {
		font-size: 16px;
		color: #666;
		margin: 20px 0;
	}

	.archive-item__trader em {
		font-style: italic;
	}

	.archive-item__btn {
		display: inline-block;
		padding: 8px 20px;
		font-size: 13px;
		font-weight: 600;
		color: #fff;
		background: transparent linear-gradient(180deg, #17a2b8 0%, #138496 100%) 0% 0% no-repeat padding-box;
		border-radius: 50px;
		text-decoration: none;
		transition: all 0.2s;
		border: none;
	}

	.archive-item__btn:hover {
		background: transparent linear-gradient(180deg, #138496 0%, #0f6674 100%) 0% 0% no-repeat padding-box;
	}

	/* Pagination */
	.pagination {
		text-align: center;
		margin-top: 20px;
	}

	.pagination__number {
		display: inline-block;
		padding: 8px 12px;
		font-size: 14px;
		font-weight: 600;
		color: #666;
		background: #fff;
		text-decoration: none;
		border: 1px solid #e0e0e0;
		margin: 0 -1px;
	}

	.pagination__number--active {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	.pagination__number:first-child {
		border-radius: 4px 0 0 4px;
	}

	.pagination__number:last-child,
	.pagination__next:last-child {
		border-radius: 0 4px 4px 0;
	}

	.pagination__dots {
		display: inline-block;
		padding: 8px 12px;
		font-size: 14px;
		color: #666;
	}

	.pagination__next {
		display: inline-block;
		padding: 8px 12px;
		font-size: 14px;
		color: #666;
		background: #fff;
		text-decoration: none;
		border: 1px solid #e0e0e0;
		margin: 0 -1px;
	}

	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__header,
		.archive-section {
			padding: 15px 20px;
		}

		.archive-item {
			padding: 15px 20px;
		}

		.archive-item__title {
			font-size: 16px;
		}
	}

	/* Featured Section - WordPress Match */
	.u--background-color-white {
		background-color: #fff;
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

	.section-title-alt {
		font-size: 28px;
		margin-bottom: 20px;
	}

	.section-title-alt--underline {
		border-bottom: 3px solid #0984ae;
		padding-bottom: 10px;
		display: inline-block;
	}

	.u--font-weight-bold {
		font-weight: 700;
	}

	.u--hide-read-more p {
		margin: 10px 0 20px;
	}

	.u--border-radius {
		border-radius: 8px;
		width: 100%;
	}

	.hidden-md {
		display: block;
	}

	.d-lg-none {
		display: block;
	}

	.d-none {
		display: none;
	}

	.d-lg-block {
		display: block;
	}

	.pb-2 {
		padding-bottom: 0.5rem;
	}

	@media (min-width: 992px) {
		.d-lg-none {
			display: none;
		}

		.d-lg-block {
			display: block;
		}

		.col-lg-5 {
			flex: 0 0 41.666667%;
			max-width: 41.666667%;
		}

		.col-lg-7 {
			flex: 0 0 58.333333%;
			max-width: 58.333333%;
		}
	}

	@media (max-width: 576px) {
		.col-sm-6 {
			flex: 0 0 100%;
			max-width: 100%;
		}

		.hidden-xs {
			display: none;
		}
	}
</style>
