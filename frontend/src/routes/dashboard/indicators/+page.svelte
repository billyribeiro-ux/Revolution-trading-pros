<!--
	My Indicators Page
	═══════════════════════════════════════════════════════════════════════════
	Pixel-perfect match to WordPress implementation
	Reference: MyIndicatorsY and MyIndicatorsN HTML files
	
	Features:
	- Shows indicators when user has access
	- Empty state when user has no indicators
	- Breadcrumb: Home / Member Dashboard / My Indicators
	- Page title with dashboard__header wrapper
	- Indicator cards with platform links
	- Matches original WordPress structure exactly
-->
<script lang="ts">
	import DashboardBreadcrumbs from '$lib/components/dashboard/DashboardBreadcrumbs.svelte';
	
	interface Indicator {
		id: number;
		name: string;
		description: string;
		platform: string;
		platformUrl: string;
		icon: string;
		status: 'active' | 'inactive';
	}
	
	// Pagination configuration - ICT 7 Enterprise Standard
	let currentPage = 1;
	const itemsPerPage = 12;
	
	// Mock data - replace with actual API call
	// Set to empty array to show empty state, or populate with indicators
	// All 53 indicators from WordPress my-indicators-yes reference
	const indicators: Indicator[] = [
		{ id: 1, name: 'Volume Max Tool Kit (formerly VWAP)', description: '', platform: 'TradingView', platformUrl: '/indicators/volume-max-i', icon: '', status: 'active' },
		{ id: 2, name: 'Multi (EMA) Cross Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-cross-i', icon: '', status: 'active' },
		{ id: 3, name: 'QuantPivots Tool', description: '', platform: 'TradingView', platformUrl: '/indicators/quantpivots-i', icon: '', status: 'active' },
		{ id: 4, name: 'Trend and HiLo Oscillator Pro Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/trend-hilo-oscillator-pro-i', icon: '', status: 'active' },
		{ id: 5, name: 'Darvas Box 3.0', description: '', platform: 'TradingView', platformUrl: '/indicators/darvas-3-i', icon: '', status: 'active' },
		{ id: 6, name: 'Ready.Aim.Fire!® (RAF) Pro Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/ready-aim-fire-pro-i', icon: '', status: 'active' },
		{ id: 7, name: 'Multi Squeeze Pro Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-squeeze-pro-i', icon: '', status: 'active' },
		{ id: 8, name: 'TurboVZO Indicator & VZO Signals', description: '', platform: 'TradingView', platformUrl: '/indicators/turbovzo-indicator-i', icon: '', status: 'active' },
		{ id: 9, name: 'Multi 10x', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-10x-i', icon: '', status: 'active' },
		{ id: 10, name: 'Power Correlations', description: '', platform: 'TradingView', platformUrl: '/indicators/power-correlations', icon: '', status: 'active' },
		{ id: 11, name: 'GRaB and Wave Premium', description: '', platform: 'TradingView', platformUrl: '/indicators/grab-and-wave-premium-i', icon: '', status: 'active' },
		{ id: 12, name: 'Top Hat Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/top-hat-indicator', icon: '', status: 'active' },
		{ id: 13, name: 'Multi-Squeeze Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-squeeze-indicator', icon: '', status: 'active' },
		{ id: 14, name: 'Compound Breakout Tool', description: '', platform: 'TradingView', platformUrl: '/indicators/compound-breakout-tool-i', icon: '', status: 'active' },
		{ id: 15, name: 'Divergent Bar', description: '', platform: 'TradingView', platformUrl: '/indicators/divergent-bar-i', icon: '', status: 'active' },
		{ id: 16, name: 'Reversal Arrows (formerly HOLB/LOHB)', description: '', platform: 'TradingView', platformUrl: '/indicators/reversal-arrows-holblohb-i', icon: '', status: 'active' },
		{ id: 17, name: 'Voodoo Lines®', description: '', platform: 'TradingView', platformUrl: '/indicators/voodoo-lines-i', icon: '', status: 'active' },
		{ id: 18, name: 'Two Week Statistics Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/two-week-statistics-i', icon: '', status: 'active' },
		{ id: 19, name: 'Weekly Price Statistics Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/weekly-price-statistics-i', icon: '', status: 'active' },
		{ id: 20, name: 'Cumulative and Comparative TICK', description: '', platform: 'TradingView', platformUrl: '/indicators/cumulative-comparative-tick-i', icon: '', status: 'active' },
		{ id: 21, name: 'Multi Time Frame (MTF) Trend', description: '', platform: 'TradingView', platformUrl: '/indicators/mtf-trend-i', icon: '', status: 'active' },
		{ id: 22, name: 'Credit Sniper', description: '', platform: 'TradingView', platformUrl: '/indicators/credit-sniper-i', icon: '', status: 'active' },
		{ id: 23, name: 'Earnings Indicator', description: '', platform: 'TradingView', platformUrl: '/indicators/earnings-indicator-i', icon: '', status: 'active' },
		{ id: 24, name: '10x Bars', description: '', platform: 'TradingView', platformUrl: '/indicators/10x-bars-i', icon: '', status: 'active' },
		{ id: 25, name: 'Probability Zones', description: '', platform: 'TradingView', platformUrl: '/indicators/probability-zones-i', icon: '', status: 'active' },
		{ id: 26, name: 'Launch Pad', description: '', platform: 'TradingView', platformUrl: '/indicators/launch-pad-i', icon: '', status: 'active' },
		{ id: 27, name: 'Squeeze Pro Stats', description: '', platform: 'TradingView', platformUrl: '/indicators/squeeze-pro-stats-i', icon: '', status: 'active' },
		{ id: 28, name: 'EINO Pro', description: '', platform: 'TradingView', platformUrl: '/indicators/eino-pro-i', icon: '', status: 'active' },
		{ id: 29, name: 'Squeeze Pro', description: '', platform: 'TradingView', platformUrl: '/indicators/squeeze-pro-i', icon: '', status: 'active' },
		{ id: 30, name: 'Dynamic Profit Zones', description: '', platform: 'TradingView', platformUrl: '/indicators/dynamic-profit-zones-i', icon: '', status: 'active' },
		{ id: 31, name: 'Phoenix Finder', description: '', platform: 'TradingView', platformUrl: '/indicators/phoenix-finder-i', icon: '', status: 'active' },
		{ id: 32, name: 'Power Correlations TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/power-correlations-tsd', icon: '', status: 'active' },
		{ id: 33, name: 'Multi-Squeeze Indicator TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-squeeze-indicator-tsd', icon: '', status: 'active' },
		{ id: 34, name: 'Top Hat Indicator TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/top-hat-indicator-tsd', icon: '', status: 'active' },
		{ id: 35, name: 'Compound Breakout Tool TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/compound-breakout-tool-tsd', icon: '', status: 'active' },
		{ id: 36, name: 'Divergent Bar TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/divergent-bar-tsd', icon: '', status: 'active' },
		{ id: 37, name: 'Reversal Arrows TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/reversal-arrows-holblohb-tsd', icon: '', status: 'active' },
		{ id: 38, name: '10x Bars TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/10x-bars-tsd', icon: '', status: 'active' },
		{ id: 39, name: 'Ready.Aim.Fire! Pro TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/ready-aim-fire-pro-tsd', icon: '', status: 'active' },
		{ id: 40, name: 'Squeeze Pro Stats TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/squeeze-pro-stats-tsd', icon: '', status: 'active' },
		{ id: 41, name: 'EINO Pro TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/eino-pro-tsd', icon: '', status: 'active' },
		{ id: 42, name: 'Squeeze Pro TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/squeeze-pro-tsd', icon: '', status: 'active' },
		{ id: 43, name: 'Dynamic Profit Zones Activated', description: '', platform: 'TradingView', platformUrl: '/indicators/dynamic-profit-zones-i-activated', icon: '', status: 'active' },
		{ id: 44, name: 'Multi Cross Activated', description: '', platform: 'TradingView', platformUrl: '/indicators/multi-cross-i-activated', icon: '', status: 'active' },
		{ id: 45, name: 'VX TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/vx-tsd', icon: '', status: 'active' },
		{ id: 46, name: 'Probability Zones TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/probability-zones-tsd', icon: '', status: 'active' },
		{ id: 47, name: 'Launch Pad TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/launch-pad-tsd', icon: '', status: 'active' },
		{ id: 48, name: 'Two Week TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/twoweek-tsd', icon: '', status: 'active' },
		{ id: 49, name: 'Phoenix Finder TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/phoenix-finder-tsd', icon: '', status: 'active' },
		{ id: 50, name: 'Credit Sniper TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/credit-sniper-tsd', icon: '', status: 'active' },
		{ id: 51, name: 'Weekly Statistics TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/weekly-statistics-tsd', icon: '', status: 'active' },
		{ id: 52, name: 'CCTICK TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/cctick-tsd', icon: '', status: 'active' },
		{ id: 53, name: 'TurboVZO TSD', description: '', platform: 'TradingView', platformUrl: '/indicators/turbovzo-tsd', icon: '', status: 'active' }
	];
	
	// Computed pagination values
	$: totalPages = Math.ceil(indicators.length / itemsPerPage);
	$: startIndex = (currentPage - 1) * itemsPerPage;
	$: endIndex = startIndex + itemsPerPage;
	$: paginatedIndicators = indicators.slice(startIndex, endIndex);
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
	<title>My Indicators - Simpler Trading</title>
	<meta name="description" content="Access your trading indicators and tools" />
	<meta property="og:title" content="My Indicators" />
	<meta property="og:url" content="https://my.simplertrading.com/dashboard/indicators" />
	<meta property="og:type" content="article" />
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
					<h1 class="dashboard__page-title">My Indicators</h1>
				</div>
			</header>
			
			<!-- Indicators Grid or Empty State -->
			{#if indicators.length === 0}
				<!-- Empty State -->
				<div class="dashboard__content">
					<div class="dashboard__content-main">
						<section class="dashboard__content-section">
							<div class="empty-state">
								<p class="empty-state__message">You don't have any Indicators.</p>
								<a href="https://www.simplertrading.com/product/product-category/indicators/" class="btn btn-orange" target="_blank">See All Indicators</a>
							</div>
						</section>
					</div>
				</div>
			{:else}
				<!-- Indicators Grid -->
				<div class="dashboard__content">
					<div class="dashboard__content-main">
						<section class="dashboard__content-section">
							<div class="card-grid flex-grid row">
								{#each paginatedIndicators as indicator}
									<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
										<div class="card flex-grid-panel">
											<section class="card-body u--squash">
												<h4 class="h5 card-title pb-1">
													<a href="{indicator.platformUrl}">
														{indicator.name}
													</a>
												</h4>
												{#if indicator.description}
													<p class="article-card__meta">
														<small>{indicator.description}</small>
													</p>
												{/if}
											</section>
											<footer class="card-footer">
												<a class="btn btn-tiny btn-default" href="{indicator.platformUrl}">Watch Now</a>
											</footer>
										</div>
									</article>
								{/each}
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
	 * My Indicators Page Styles
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
		margin-bottom: 30px;
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.card-grid-spacer {
		padding-left: 15px;
		padding-right: 15px;
		margin-top: 30px;
		margin-bottom: 30px;
	}

	.flex-grid-item {
		display: flex;
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
	}

	.btn-tiny {
		font-size: 14px;
		line-height: 16px;
		padding: 6px 8px;
		font-weight: 600;
	}

	.btn-default {
		background: #f4f4f4;
		color: #0984ae;
		border-color: transparent;
		box-shadow: none;
		transition: all 0.15s ease-in-out;
	}

	.btn-default:visited {
		color: #0984ae;
	}

	.btn-default:hover {
		color: #0984ae;
		background: #e7e7e7;
		border-color: transparent;
		box-shadow: none;
		text-decoration: none;
	}

	.btn-default:focus,
	.btn-default:active {
		color: #0984ae;
		background: #e7e7e7;
		border-color: transparent;
		outline: 0;
		box-shadow: 0 0 0 0.25rem rgba(13, 110, 253, 0.25);
		text-decoration: none;
	}

	/* Grid System - Bootstrap-like */
	.row {
		display: flex;
		flex-wrap: wrap;
		margin-left: -15px;
		margin-right: -15px;
	}

	.col-xs-12 {
		width: 100%;
	}

	@media (min-width: 576px) {
		.col-sm-6 {
			width: 50%;
		}
	}

	@media (min-width: 768px) {
		.col-md-6 {
			width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-lg-4 {
			width: 33.333333%;
		}
	}
	
	/* Responsive */
	@media (max-width: 768px) {
		.dashboard__page-title {
			font-size: 24px;
		}

		.card-grid-spacer {
			margin-bottom: 20px;
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
