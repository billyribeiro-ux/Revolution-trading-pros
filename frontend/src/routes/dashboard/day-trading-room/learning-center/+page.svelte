<!--
	URL: /dashboard/day-trading-room/learning-center

	Learning Center Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation

	Educational resources and training materials for Day Trading Room members.
	Matches WordPress Learning Center implementation exactly.

	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { onMount } from 'svelte';

	interface LearningResource {
		id: number;
		title: string;
		trader: string;
		excerpt: string;
		slug: string;
		thumbnail: string;
		categories: string[];
	}

	// Filter state
	let activeFilter = 'all';
	let filteredResources: LearningResource[] = [];

	// Pagination state
	let currentPage = 1;
	const itemsPerPage = 9;
	let totalPages = 1;
	let paginatedResources: LearningResource[] = [];

	// Category options matching WordPress
	const categories = [
		{ id: 'all', label: 'All' },
		{ id: 'methodology', label: 'Methodology' },
		{ id: 'trade-setups', label: 'Trade Setups & Strategies' },
		{ id: 'member-webinar', label: 'Member Webinar' },
		{ id: 'trade-management', label: 'Trade & Money Management' },
		{ id: 'indicators', label: 'Indicators & Tools' }
	];

	// Learning resources data - matches WordPress structure
	const allResources: LearningResource[] = [
		{
			id: 1,
			title: 'Q3 Market Outlook July 2025',
			trader: 'John Carter',
			excerpt: "Using the economic cycle, John Carter will share insights on what's next in the stock market, commodities, Treasury yields, bonds, and more.",
			slug: 'market-outlook-jul2025-john-carter',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['trade-setups']
		},
		{
			id: 2,
			title: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			trader: 'Kody Ashmore',
			excerpt: "Intro to Kody Ashmore's Daily Sessions (and My Results!)",
			slug: 'kody-ashmore-daily-sessions-results',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			categories: ['methodology']
		},
		{
			id: 3,
			title: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			trader: 'Chris Brecher',
			excerpt: 'The 15:50 Trade (How Buybacks Matter the Last 10 Minutes Every Day)',
			slug: '15-50-trade',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			categories: ['member-webinar', 'trade-management']
		},
		{
			id: 4,
			title: 'How Mike Teeto Builds His Watchlist',
			trader: 'Mike Teeto',
			excerpt: 'How Mike Teeto Builds His Watchlist',
			slug: 'mike-teeto-watchlist',
			thumbnail: 'https://cdn.simplertrading.com/2024/10/18134533/LearningCenter_MT.jpg',
			categories: ['member-webinar']
		},
		{
			id: 5,
			title: 'Understanding Market Structure',
			trader: 'Henry Gambell',
			excerpt: 'Learn how to identify key market structure levels and use them in your trading.',
			slug: 'understanding-market-structure',
			thumbnail: 'https://cdn.simplertrading.com/2025/05/07134745/SimplerCentral_HG.jpg',
			categories: ['methodology', 'trade-setups']
		},
		{
			id: 6,
			title: 'Options Trading Fundamentals',
			trader: 'John Carter',
			excerpt: 'Master the basics of options trading with this comprehensive guide.',
			slug: 'options-trading-fundamentals',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['methodology']
		},
		{
			id: 7,
			title: 'Using Squeeze Pro Indicator',
			trader: 'John Carter',
			excerpt: 'Learn how to use the Squeeze Pro indicator to identify high-probability trade setups.',
			slug: 'squeeze-pro-indicator',
			thumbnail: 'https://cdn.simplertrading.com/dev/wp-content/uploads/2018/11/27111943/MemberWebinar-John.jpg',
			categories: ['indicators']
		},
		{
			id: 8,
			title: 'Risk Management Essentials',
			trader: 'Chris Brecher',
			excerpt: 'Protect your capital with proven risk management strategies.',
			slug: 'risk-management-essentials',
			thumbnail: 'https://cdn.simplertrading.com/2022/10/10141416/Chris-Member-Webinar.jpg',
			categories: ['trade-management']
		},
		{
			id: 9,
			title: 'Futures Trading 101',
			trader: 'Kody Ashmore',
			excerpt: 'Get started with futures trading - from basics to advanced strategies.',
			slug: 'futures-trading-101',
			thumbnail: 'https://cdn.simplertrading.com/2022/12/18125338/Kody.jpg',
			categories: ['methodology', 'trade-setups']
		}
	];

	function filterResources(categoryId: string) {
		activeFilter = categoryId;
		currentPage = 1; // Reset to first page when filter changes
		if (categoryId === 'all') {
			filteredResources = allResources;
		} else {
			filteredResources = allResources.filter(r => r.categories.includes(categoryId));
		}
		updatePagination();
	}

	function updatePagination() {
		totalPages = Math.ceil(filteredResources.length / itemsPerPage);
		const startIndex = (currentPage - 1) * itemsPerPage;
		const endIndex = startIndex + itemsPerPage;
		paginatedResources = filteredResources.slice(startIndex, endIndex);
	}

	function goToPage(page: number) {
		if (page >= 1 && page <= totalPages) {
			currentPage = page;
			updatePagination();
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function getPaginationRange(): (number | string)[] {
		const range: (number | string)[] = [];
		const delta = 1;

		if (totalPages <= 7) {
			for (let i = 1; i <= totalPages; i++) {
				range.push(i);
			}
		} else {
			// Always show first page
			range.push(1);

			if (currentPage > 3) {
				range.push('...');
			}

			// Pages around current
			for (let i = Math.max(2, currentPage - delta); i <= Math.min(totalPages - 1, currentPage + delta); i++) {
				if (!range.includes(i)) {
					range.push(i);
				}
			}

			if (currentPage < totalPages - 2) {
				range.push('...');
			}

			// Always show last page
			if (!range.includes(totalPages)) {
				range.push(totalPages);
			}
		}

		return range;
	}

	function getCategoryLabel(categoryId: string): string {
		const category = categories.find(c => c.id === categoryId);
		return category?.label || categoryId;
	}

	onMount(() => {
		filterResources('all');
		updatePagination();
	});
</script>

<svelte:head>
	<title>Learning Center | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access educational resources, training materials, and courses for day trading mastery." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Learning Center</h1>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filter Buttons -->
		<div class="lc-filter-buttons">
			{#each categories as category}
				<button
					type="button"
					class="lc-filter-btn"
					class:active={activeFilter === category.id}
					on:click={() => filterResources(category.id)}
				>
					{category.label}
				</button>
			{/each}
		</div>

		<!-- Results Count -->
		<div class="lc-results-count">
			Showing <strong>{filteredResources.length}</strong> {filteredResources.length === 1 ? 'resource' : 'resources'}
		</div>

		<!-- Learning Resources Grid -->
		<div class="lc-grid row">
			{#each paginatedResources as resource (resource.id)}
				<div class="col-xs-12 col-sm-6 col-md-6 col-xl-4 flex-grid-item">
					<article class="article-card">
						<figure 
							class="article-card__image" 
							style="background-image: url({resource.thumbnail});"
						>
							<img 
								src="https://cdn.simplertrading.com/2019/01/14105015/generic-video-card-min.jpg" 
								alt={resource.title}
								loading="lazy"
							/>
						</figure>
						
						<div class="article-card__type">
							{#each resource.categories as cat}
								<span class="label label--info">{getCategoryLabel(cat)}</span>
							{/each}
						</div>
						
						<h4 class="h5 article-card__title">
							<a href="/learning-center/{resource.slug}">{resource.title}</a>
						</h4>
						
						<div class="u--margin-top-0">
							<span class="trader_name"><i>With {resource.trader}</i></span>
						</div>
						
						<div class="article-card__excerpt u--hide-read-more">
							<p>{resource.excerpt}</p>
						</div>
						
						<a href="/learning-center/{resource.slug}" class="btn btn-tiny btn-default">
							Watch Now
						</a>
					</article>
				</div>
			{/each}
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="fl-builder-pagination">
				<ul class="page-numbers">
					{#if currentPage > 1}
						<li>
							<button 
								class="page-numbers" 
								on:click={() => goToPage(currentPage - 1)}
								type="button"
								aria-label="Previous page"
							>
								&laquo;
							</button>
						</li>
					{/if}
					
					{#each getPaginationRange() as pageNum}
						<li>
							{#if pageNum === '...'}
								<span class="page-numbers dots">…</span>
							{:else if pageNum === currentPage}
								<span class="page-numbers current" aria-current="page">{pageNum}</span>
							{:else}
								<button 
									class="page-numbers" 
									on:click={() => goToPage(Number(pageNum))}
									type="button"
									aria-label="Go to page {pageNum}"
								>
									{pageNum}
								</button>
							{/if}
						</li>
					{/each}
					
					{#if currentPage < totalPages}
						<li>
							<button 
								class="page-numbers" 
								on:click={() => goToPage(currentPage + 1)}
								type="button"
								aria-label="Next page"
							>
								&raquo;
							</button>
						</li>
					{/if}
				</ul>
			</div>
		{/if}
	</div>
</div>

<style>
	/* Filter Buttons */
	.lc-filter-buttons {
		display: flex;
		flex-wrap: wrap;
		gap: 10px;
		margin-bottom: 25px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 4px;
	}

	.lc-filter-btn {
		padding: 8px 16px;
		border: 1px solid #ddd;
		background: #fff;
		color: #333;
		font-size: 13px;
		font-weight: 600;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: inherit;
	}

	.lc-filter-btn:hover {
		background: #e9e9e9;
		border-color: #ccc;
	}

	.lc-filter-btn.active {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	/* Results Count */
	.lc-results-count {
		margin-bottom: 20px;
		font-size: 14px;
		color: #666;
	}

	/* Grid Layout */
	.lc-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.flex-grid-item {
		padding: 0 15px 30px;
		display: flex;
	}

	/* Article Card */
	.article-card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
		padding: 0 0 20px;
	}

	.article-card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.article-card__image {
		position: relative;
		width: 100%;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		margin: 0;
		overflow: hidden;
	}

	.article-card__image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		object-fit: cover;
		opacity: 0;
	}

	.article-card__type {
		padding: 15px 20px 10px;
		display: flex;
		flex-wrap: wrap;
		gap: 6px;
	}

	.label {
		display: inline-block;
		padding: 4px 10px;
		font-size: 11px;
		font-weight: 600;
		text-transform: uppercase;
		border-radius: 3px;
	}

	.label--info {
		background: #e8f4fc;
		color: #0984ae;
	}

	.article-card__title {
		padding: 0 20px;
		margin: 0 0 10px;
		font-size: 16px;
		font-weight: 700;
		line-height: 1.4;
	}

	.article-card__title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.article-card__title a:hover {
		color: #F69532;
	}

	.u--margin-top-0 {
		padding: 0 20px;
		margin-top: 0;
	}

	.trader_name {
		font-size: 13px;
		color: #666;
	}

	.trader_name i {
		font-style: italic;
	}

	.article-card__excerpt {
		padding: 10px 20px;
		flex: 1;
	}

	.article-card__excerpt p {
		margin: 0;
		font-size: 14px;
		color: #666;
		line-height: 1.5;
	}

	.u--hide-read-more {
		overflow: hidden;
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
	}

	/* Button */
	.btn {
		display: inline-block;
		margin: 10px 20px 0;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		align-self: flex-start;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background: #F69532;
		color: #fff;
		border: none;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Pagination */
	.fl-builder-pagination {
		margin-top: 40px;
		padding: 20px;
		background: #f9f9f9;
		border-radius: 4px;
		text-align: center;
	}

	.fl-builder-pagination ul.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: inline-flex;
		flex-wrap: wrap;
		gap: 0;
	}

	.fl-builder-pagination li {
		display: inline-block;
		list-style: none;
		margin: 0;
		padding: 0;
	}

	.fl-builder-pagination li button.page-numbers,
	.fl-builder-pagination li span.page-numbers {
		border: 1px solid #e6e6e6;
		display: inline-block;
		padding: 10px 16px;
		margin: 0;
		min-width: 45px;
		text-align: center;
		text-decoration: none;
		color: #1e73be;
		background: #fff;
		transition: all 0.2s ease;
		cursor: pointer;
		font-family: inherit;
		font-size: 16px;
		font-weight: 400;
	}

	.fl-builder-pagination li button.page-numbers:hover {
		background: #f5f5f5;
		border-color: #ccc;
	}

	.fl-builder-pagination li span.current {
		background: #1e73be;
		color: #fff;
		border-color: #1e73be;
		font-weight: 600;
	}

	.fl-builder-pagination li span.dots {
		background: #fff;
		cursor: default;
		color: #333;
	}

	/* Responsive Grid */
	@media (max-width: 1199px) {
		.col-xl-4 {
			width: 50%;
		}
	}

	@media (max-width: 767px) {
		.col-sm-6 {
			width: 50%;
		}
		
		.lc-filter-buttons {
			padding: 15px;
			gap: 8px;
		}
		
		.lc-filter-btn {
			padding: 6px 12px;
			font-size: 12px;
		}
	}

	@media (max-width: 575px) {
		.col-xs-12 {
			width: 100%;
		}
		
		.lc-grid {
			margin: 0 -10px;
		}
		
		.flex-grid-item {
			padding: 0 10px 20px;
		}
	}

	@media (min-width: 1200px) {
		.col-xl-4 {
			width: 33.333%;
		}
	}
</style>
