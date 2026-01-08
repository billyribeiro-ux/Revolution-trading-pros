<!--
	My Classes List Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyClassesY HTML file
	
	Features:
	- Grid layout of available classes
	- Breadcrumb: Home / Member Dashboard / My Classes
	- Page title with dashboard__header wrapper
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	
	interface ClassItem {
		id: number;
		title: string;
		slug: string;
		description?: string;
		thumbnail?: string;
		duration?: string;
		instructor?: string;
		date?: string;
	}
	
	// Pagination configuration - ICT 7 Enterprise Standard
	let currentPage = 1;
	const itemsPerPage = 12;
	
	// Mock data - replace with actual API call
	// Set to empty array to show empty state, or populate with classes
	const classes: ClassItem[] = [
		{
			id: 1,
			title: 'Tax Loss Harvest',
			slug: 'tax-loss-harvest-c',
			date: 'December 2025',
			instructor: 'John Carter'
		},
		{
			id: 2,
			title: 'Mastering the Trade University',
			slug: 'mtt-university-c',
			date: 'December 2025',
			instructor: 'John Carter'
		},
		{
			id: 3,
			title: 'Quickstart to Precision Trading E-Learning Module',
			slug: 'quickstart-precision-trading-elearning-c',
			date: 'February 2022',
			instructor: 'TG Watkins'
		},
		{
			id: 4,
			title: 'Quickstart To Precision Trading',
			slug: 'quickstart-precision-trading-c',
			date: 'April 2020',
			instructor: 'TG Watkins'
		},
		{
			id: 5,
			title: 'Advanced Options Strategies',
			slug: 'advanced-options-strategies-c',
			date: 'November 2024',
			instructor: 'John Carter'
		},
		{
			id: 6,
			title: 'Technical Analysis Masterclass',
			slug: 'technical-analysis-masterclass-c',
			date: 'October 2024',
			instructor: 'TG Watkins'
		},
		{
			id: 7,
			title: 'Risk Management Essentials',
			slug: 'risk-management-essentials-c',
			date: 'September 2024',
			instructor: 'John Carter'
		},
		{
			id: 8,
			title: 'Market Psychology & Trading',
			slug: 'market-psychology-trading-c',
			date: 'August 2024',
			instructor: 'TG Watkins'
		},
		{
			id: 9,
			title: 'Swing Trading Fundamentals',
			slug: 'swing-trading-fundamentals-c',
			date: 'July 2024',
			instructor: 'John Carter'
		}
	];
	
	// Computed pagination values
	$: totalPages = Math.ceil(classes.length / itemsPerPage);
	$: startIndex = (currentPage - 1) * itemsPerPage;
	$: endIndex = startIndex + itemsPerPage;
	$: paginatedClasses = classes.slice(startIndex, endIndex);
	$: pageNumbers = Array.from({ length: totalPages }, (_, i) => i + 1);
	
	// Pagination navigation functions
	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}
	
	function nextPage() {
		if (currentPage < totalPages) {
			goToPage(currentPage + 1);
		}
	}
	
	function previousPage() {
		if (currentPage > 1) {
			goToPage(currentPage - 1);
		}
	}
</script>

<svelte:head>
	<title>My Classes - Simpler Trading</title>
	<meta name="description" content="Access your trading classes and educational content" />
</svelte:head>

<!-- Breadcrumbs -->
<DashboardBreadcrumbs />

<!-- Main Content -->
<div id="page" class="hfeed site grid-parent">
	<div id="content" class="site-content">
		<aside class="dashboard__sidebar">
			<!-- Sidebar content if needed -->
		</aside>
		
		<main class="dashboard__main">
			<!-- Page Header with dashboard__header wrapper (matches WordPress) -->
			<header class="dashboard__header">
				<div class="dashboard__header-left">
					<h1 class="dashboard__page-title">My Classes</h1>
				</div>
			</header>
			
			<!-- Classes Grid or Empty State -->
		{#if classes.length === 0}
			<!-- Empty State -->
			<div class="dashboard__content">
				<div class="dashboard__content-main">
					<section class="dashboard__content-section">
						<div class="empty-state">
							<p class="empty-state__message">You don't have any Classes.</p>
							<a href="/courses" class="btn btn-orange">See All Courses</a>
						</div>
					</section>
				</div>
			</div>
		{:else}
			<!-- Classes Grid -->
			<div class="dashboard__content">
				<div class="dashboard__content-main">
					<section class="dashboard__content-section">
						<div>
							<div class="card-grid flex-grid row">
								{#each paginatedClasses as classItem}
									<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
										<div class="card flex-grid-panel">
											<section class="card-body u--squash">
												<h4 class="h5 card-title pb-1">
													<a href="/classes/{classItem.slug}">
														{classItem.title}
													</a>
												</h4>
												{#if classItem.date && classItem.instructor}
													<p class="article-card__meta">
														<small>{classItem.date} with {classItem.instructor}</small>
													</p>
												{/if}
											</section>
											<footer class="card-footer">
												<a class="btn btn-tiny btn-default" href="/classes/{classItem.slug}">Watch Now</a>
											</footer>
										</div>
									</article>
								{/each}
							</div>
						</div>
						
						<!-- Pagination Controls -->
						{#if totalPages > 1}
							<div class="pagination-wrapper">
								<nav class="pagination" aria-label="Pagination">
									<ul class="page-numbers">
										<!-- Previous Button -->
										<li>
											<button
												class="page-numbers prev"
												on:click={previousPage}
												disabled={currentPage === 1}
												aria-label="Previous page"
											>
												← Previous
											</button>
										</li>
										
										<!-- Page Numbers -->
										{#each pageNumbers as pageNum}
											<li>
												{#if pageNum === currentPage}
													<span class="page-numbers current" aria-current="page">{pageNum}</span>
												{:else}
													<button
														class="page-numbers"
														on:click={() => goToPage(pageNum)}
														aria-label="Go to page {pageNum}"
													>
														{pageNum}
													</button>
												{/if}
											</li>
										{/each}
										
										<!-- Next Button -->
										<li>
											<button
												class="page-numbers next"
												on:click={nextPage}
												disabled={currentPage === totalPages}
												aria-label="Next page"
											>
												Next →
											</button>
										</li>
									</ul>
								</nav>
							</div>
						{/if}
					</section>
				</div>
			</div>
		{/if}
		</main>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * My Classes Page Styles
	 * Matches WordPress implementation exactly
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		justify-content: space-between;
		background: #fff;
		border-bottom: 1px solid #dbdbdb;
		border-right: 1px solid #dbdbdb;
		padding: 20px;
	}
	
	@media (min-width: 1280px) {
		.dashboard__header {
			padding: 30px;
		}
	}

	@media (min-width: 1440px) {
		.dashboard__header {
			padding: 30px 40px;
		}
	}
	
	.dashboard__header-left {
		flex: 1;
	}
	
	.dashboard__page-title {
		font-size: 28px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	/* Empty State - Matches screenshot */
	.empty-state {
		background: #f5f5f5;
		padding: 40px 20px;
		text-align: left;
		min-height: 200px;
	}

	.empty-state__message {
		font-size: 16px;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	.btn-orange {
		background-color: #ff8c00;
		color: #fff;
		padding: 12px 24px;
		border-radius: 4px;
		text-decoration: none;
		display: inline-block;
		font-weight: 700;
		font-size: 14px;
		border: none;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-orange:hover {
		background-color: #e67e00;
		text-decoration: none;
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * Card Grid - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
		margin-bottom: 30px;
	}

	.card-grid-spacer {
		padding-left: 15px;
		padding-right: 15px;
		margin-top: 30px;
		box-sizing: border-box;
	}

	.flex-grid-item {
		display: flex;
		flex-direction: column;
	}

	.card {
		position: relative;
		background: #fff;
		border-radius: 5px;
		box-shadow: 0 1px 2px rgba(0, 0, 0, 0.15);
		display: flex;
		flex-direction: column;
		width: 100%;
		height: 100%;
		transition: all 0.15s ease-in-out;
		padding-bottom: 60px;
	}

	.card:hover {
		box-shadow: 0 5px 20px rgba(0, 0, 0, 0.1);
	}

	.flex-grid-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		width: 100%;
	}

	.card-body {
		padding: 20px;
		flex-grow: 0;
		display: flex;
		flex-direction: column;
		text-align: center;
		align-items: center;
	}

	.card-body:last-of-type {
		flex-grow: 1;
	}

	.card-body + .card-body {
		border-top: 1px solid #e8e8e8;
	}

	.u--squash {
		padding-bottom: 10px;
	}

	.card-title {
		font-size: 18px;
		font-weight: 700;
		color: #333;
		margin: 0;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		line-height: 1.4;
		text-align: center;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s;
		display: block;
		text-align: center;
	}

	.card-title a:hover {
		color: #143E59;
	}

	.h5 {
		font-size: 18px;
		font-weight: 600;
	}

	.pb-1 {
		padding-bottom: 0.5rem;
	}

	.article-card__meta {
		color: #999;
		font-size: 13px;
		margin: 8px 0 0;
		line-height: 1.5;
		text-align: center;
	}

	.article-card__meta small {
		font-size: 13px;
	}

	.card-footer {
		position: absolute;
		bottom: 0;
		left: 0;
		right: 0;
		padding: 0 20px 20px;
		text-align: center;
		display: flex;
		justify-content: center;
	}

	.btn {
		display: inline-block;
		text-decoration: none;
		border-radius: 5px;
		font-weight: 700;
		font-family: 'Open Sans', sans-serif;
		transition: all 0.2s ease-in-out;
		text-align: center;
		cursor: pointer;
		border: none;
		line-height: 1.5;
	}

	.btn-tiny {
		padding: 10px 24px;
		font-size: 13px;
		min-width: 120px;
	}

	.btn-default {
		background-color: #143E59;
		color: #fff;
		border: 1px solid #143E59;
		box-shadow: none;
		transition: all 0.15s ease-in-out;
	}

	.btn-default:hover {
		background-color: #0f2d41;
		border-color: #0f2d41;
		box-shadow: none;
		text-decoration: none;
		color: #fff;
	}

	/* Grid System - Bootstrap-like with Svelte Best Practices */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	/* Mobile First - Base styles for all devices */
	.col-xs-12 {
		width: 100%;
		flex: 0 0 100%;
		max-width: 100%;
	}

	/* Small devices (landscape phones, 576px and up) */
	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Medium devices (tablets, 768px and up) */
	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Large devices (desktops, 992px and up) */
	@media (min-width: 992px) {
		.col-lg-4 {
			width: 33.333333%;
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}
	
	/* Responsive Adjustments */
	@media (max-width: 991px) {
		.card-grid-spacer {
			margin-bottom: 25px;
		}
	}

	@media (max-width: 767px) {
		.dashboard__page-title {
			font-size: 24px;
		}

		.card-grid-spacer {
			margin-bottom: 20px;
		}

		.card-title {
			font-size: 16px;
		}

		.card-body {
			padding: 16px;
		}

		.card-footer {
			padding: 0 16px 16px;
		}
	}

	@media (max-width: 575px) {
		.card-grid {
			margin-left: -10px;
			margin-right: -10px;
		}

		.card-grid-spacer {
			padding-left: 10px;
			padding-right: 10px;
			margin-bottom: 15px;
		}

		.dashboard__page-title {
			font-size: 22px;
		}
	}
	
	/* ═══════════════════════════════════════════════════════════════════════════
	 * Pagination - Enterprise ICT 7 Standard
	 * ═══════════════════════════════════════════════════════════════════════════ */
	
	.pagination-wrapper {
		padding: 40px 0;
		text-align: center;
	}
	
	.pagination {
		display: inline-block;
	}
	
	.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		gap: 8px;
		align-items: center;
		justify-content: center;
		flex-wrap: wrap;
	}
	
	.page-numbers li {
		list-style: none;
		margin: 0;
		padding: 0;
	}
	
	.page-numbers button,
	.page-numbers span {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 40px;
		height: 40px;
		padding: 8px 12px;
		border: 1px solid #e6e6e6;
		background: #fff;
		color: #333;
		font-size: 14px;
		font-weight: 600;
		font-family: 'Open Sans', sans-serif;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
	}
	
	.page-numbers button:hover:not(:disabled) {
		background: #f5f5f5;
		border-color: #143E59;
		color: #143E59;
		text-decoration: none;
	}
	
	.page-numbers button:disabled {
		opacity: 0.5;
		cursor: not-allowed;
		background: #f9f9f9;
	}
	
	.page-numbers .current {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
		cursor: default;
	}
	
	.page-numbers .prev,
	.page-numbers .next {
		min-width: auto;
		padding: 8px 16px;
		font-weight: 700;
	}
	
	@media (max-width: 576px) {
		.pagination-wrapper {
			padding: 30px 0;
		}
		
		.page-numbers {
			gap: 4px;
		}
		
		.page-numbers button,
		.page-numbers span {
			min-width: 36px;
			height: 36px;
			padding: 6px 10px;
			font-size: 13px;
		}
		
		.page-numbers .prev,
		.page-numbers .next {
			padding: 6px 12px;
		}
	}
</style>
