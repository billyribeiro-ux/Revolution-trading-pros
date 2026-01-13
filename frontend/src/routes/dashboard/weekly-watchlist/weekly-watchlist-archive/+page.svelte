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
	
	let { data } = $props();
	
	const breadcrumbs = [
		{ label: 'Dashboard', href: '/dashboard' },
		{ label: 'Weekly Watchlist', href: '/dashboard/weekly-watchlist' },
		{ label: 'Archive', href: '/dashboard/weekly-watchlist/weekly-watchlist-archive' }
	];
</script>

<svelte:head>
	<title>Weekly Watchlist Archive - Revolution Trading Pros</title>
</svelte:head>

<DashboardBreadcrumbs items={breadcrumbs} />

<!-- Archive Section -->
<section class="dashboard__content-section">
	<div class="archive-container">
		<!-- Archive Grid -->
		<div class="fl-post-grid-post">
			{#each data.archiveEntries as entry}
				<div class="card fl-post-text">
					<div>
						<section class="card-body u--squash">
							<h4 class="h5 card-title">{entry.title}</h4>
							<div class="excerpt"><i>With {entry.trader}</i></div>
							<div class="fl-post-more-link">
								<a class="btn btn-tiny btn-default" href="/watchlist/{entry.slug}?tab=2">
									Read Now
								</a>
							</div>
						</section>
					</div>
				</div>
			{/each}
		</div>

		<!-- Pagination - WordPress Match -->
		<div class="text-center pagination-container">
			<div id="loopage_pg" class="pagination-wrap">
				<span aria-current="page" class="page-numbers current">{data.currentPage}</span>
				{#if data.currentPage < data.totalPages}
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=2">2</a>
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=3">3</a>
					<span class="page-numbers dots">&hellip;</span>
					<a class="page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg={data.totalPages}">{data.totalPages}</a>
					<a class="next page-numbers" href="/dashboard/weekly-watchlist/weekly-watchlist-archive?pg=2">&raquo;</a>
				{/if}
			</div>
		</div>
	</div>
</section>

<style>
	/* Archive Container */
	.archive-container {
		padding: 30px 20px;
		max-width: 1100px;
		margin: 0 auto;
	}

	/* Archive Grid - WordPress Match */
	.fl-post-grid-post {
		font-size: 19px;
		background: transparent !important;
		border: 0;
		display: flex;
		flex-wrap: wrap;
		justify-content: space-between;
		margin-bottom: 40px;
	}

	/* Archive Cards */
	.card {
		max-width: 32%;
		margin: 0.5%;
		background: #fff;
		border-radius: 8px;
		box-shadow: 0px 3px 6px rgba(0, 0, 0, 0.1);
		transition: transform 0.2s ease, box-shadow 0.2s ease;
	}

	.card:hover {
		transform: translateY(-4px);
		box-shadow: 0px 6px 12px rgba(0, 0, 0, 0.15);
	}

	.card-body {
		padding: 30px;
	}

	.u--squash {
		margin: 0;
	}

	/* Card Title */
	.card-title {
		font-size: 24px !important;
		margin-bottom: 0 !important;
		line-height: 26px;
		font-weight: 700;
		color: #333;
		font-family: 'Open Sans', sans-serif;
	}

	/* Excerpt */
	.excerpt {
		margin: 20px 0;
		max-height: 8rem;
		-webkit-box-orient: vertical;
		display: block;
		display: -webkit-box;
		overflow: hidden !important;
		text-overflow: ellipsis;
		-webkit-line-clamp: 4;
		font-size: 16px;
		color: #666;
	}

	.excerpt i {
		font-style: italic;
	}

	/* Read Now Button */
	.fl-post-more-link {
		margin-top: 15px;
	}

	.btn {
		display: inline-block;
		padding: 10px 24px;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border-radius: 50px;
		transition: all 0.2s ease;
		font-family: 'Open Sans', sans-serif;
		text-transform: uppercase;
		letter-spacing: 0.5px;
	}

	.btn-tiny {
		padding: 8px 20px;
		font-size: 13px;
	}

	.btn-default {
		background: transparent linear-gradient(180deg, #FFB834 0%, #C68000 100%) 0% 0% no-repeat padding-box;
		color: #fff !important;
		border: none;
	}

	.btn-default:hover {
		transform: translateY(-2px);
		box-shadow: 0 4px 12px rgba(255, 184, 52, 0.4);
	}

	/* Pagination - WordPress Match */
	.pagination-container {
		margin-top: 20px;
		text-align: center;
	}

	.pagination-wrap {
		display: inline-block;
		background: transparent;
	}

	.page-numbers {
		display: inline-block;
		padding: 0 10px;
		font-size: 17px;
		font-weight: 600;
		margin: 0;
		color: #666;
		background: #fff !important;
		text-align: center;
		border-radius: 0;
		text-decoration: none;
		border: 1px solid #e5e5e5;
		transition: all 0.2s ease;
	}

	.page-numbers:hover {
		background: #f5f5f5 !important;
		color: #333;
	}

	.page-numbers.current {
		font-weight: 400;
		color: #f4f4f4;
		background: #0984ae !important;
		border-color: #0984ae;
	}

	.page-numbers.dots {
		border: none;
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
		.card {
			max-width: 48%;
			margin: 1%;
		}
	}

	/* Responsive - Mobile */
	@media (max-width: 768px) {
		.archive-container {
			padding: 20px 15px;
		}

		.card {
			max-width: 100%;
			margin: 10px 0;
		}

		.card-body {
			padding: 20px;
		}

		.card-title {
			font-size: 20px !important;
		}

		.excerpt {
			font-size: 14px;
		}

		.page-numbers {
			padding: 5px 8px;
			font-size: 15px;
		}
	}
</style>
