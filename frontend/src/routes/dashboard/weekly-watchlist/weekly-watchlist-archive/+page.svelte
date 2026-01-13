<!--
	URL: /dashboard/weekly-watchlist/weekly-watchlist-archive
	
	Weekly Watchlist Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	Pixel-perfect match to WordPress reference: frontend/Implementation/wwlArchive
	
	Features:
	- Grid layout of archive cards (3 columns)
	- Pagination matching WordPress
	- Card structure matching WordPress exactly
	- Responsive design
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	
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

<DashboardBreadcrumbs />

<!-- Archive Section -->
<section class="dashboard__content-section">
		<!-- Archive Grid -->
		<div class="archive-grid">
			{#each data.archiveEntries as { slug, title, trader }}
				<article class="archive-card">
					<div class="archive-card__content">
						<h4 class="archive-card__title">{title}</h4>
						<div class="archive-card__excerpt"><i>With {trader}</i></div>
						<div class="archive-card__action">
							<a class="btn btn-tiny btn-default" href="/watchlist/{slug}?tab=2">
								Read Now
							</a>
						</div>
					</div>
				</article>
			{/each}
		</div>

		<!-- Pagination - WordPress Match -->
		<div class="text-center" style="margin-top: 20px;">
			<div id="loopage_pg" class="pagination-wrap">
				<span aria-current="page" class="page-numbers current">{data.currentPage}</span>
				{#if hasMorePages}
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=2">2</a>
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=3">3</a>
					<span class="page-numbers dots">&hellip;</span>
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg={data.totalPages}">{data.totalPages}</a>
					<a class="next page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=2">&raquo;</a>
				{/if}
			</div>
		</div>
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
	/* Archive Grid */
	.archive-grid {
		font-size: 19px;
		background: transparent !important;
		border: 0;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
	}

	/* Archive Card */
	.archive-card {
		max-width: 32%;
		margin: 0.5%;
	}

	.archive-card__content {
		padding: 0;
		margin: 0;
	}

	/* Card Title */
	.archive-card__title {
		font-size: 24px;
		margin-bottom: 0;
		line-height: 26px;
		padding: 0 0 5px;
	}

	/* Card Excerpt */
	.archive-card__excerpt {
		margin: 20px 0;
		max-height: 8rem;
		-webkit-box-orient: vertical;
		display: block;
		display: -webkit-box;
		overflow: hidden !important;
		text-overflow: ellipsis;
		-webkit-line-clamp: 4;
		line-clamp: 4;
		font-size: 16px;
	}

	.archive-card__excerpt i {
		font-style: italic;
	}

	/* Card Action */
	.archive-card__action {
		margin-top: 0;
	}

	.btn {
		display: inline-block;
		text-decoration: none;
	}

	.btn-tiny {
		padding: 8px 20px;
		font-size: 13px;
	}

	.btn-default {
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		color: #fff !important;
		border: none;
		border-radius: 50px;
	}

	/* Pagination - WordPress Match */
	.text-center {
		text-align: center;
		margin-top: 20px;
	}

	.pagination-wrap {
		background: transparent;
	}

	.page-numbers {
		background: transparent !important;
		padding: 0 10px;
		font-size: 17px;
		font-weight: 600;
		margin: 0;
		color: #666;
		background: #fff !important;
		text-align: center;
		border-radius: 0;
		text-decoration: none;
	}

	.page-numbers.current {
		font-weight: 400;
		color: #f4f4f4;
		background: #0984ae !important;
	}

	.page-numbers.dots {
		background: transparent !important;
	}

	.pagination-wrap .page-numbers:first-child {
		border-radius: 5px 0 0 5px !important;
	}

	.pagination-wrap .page-numbers:last-child {
		border-radius: 0 5px 5px 0 !important;
	}

	/* Responsive - Tablet */
	@media (max-width: 992px) {
		.archive-card {
			max-width: 48%;
			margin: 1%;
		}
	}

	/* Responsive - Mobile */
	@media (max-width: 768px) {
		.archive-card {
			max-width: 100%;
			margin: 10px 0;
		}

		.archive-card__title {
			font-size: 20px;
		}

		.archive-card__excerpt {
			font-size: 14px;
		}

		.page-numbers {
			padding: 0 8px;
			font-size: 15px;
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
