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
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';

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
	let activeFilter = $state('all');
	let filteredResources: LearningResource[] = $state([]);

	// Pagination state
	let currentPage = $state(1);
	const itemsPerPage = 9;
	let totalPages = $state(1);
	let paginatedResources: LearningResource[] = $state([]);

	// Category options matching WordPress exactly - using WordPress category IDs
	const categories = [
		{ id: '529', label: 'Trade Setups & Strategies' },
		{ id: '528', label: 'Methodology' },
		{ id: '329', label: 'Member Webinar' },
		{ id: '2932', label: 'Trade & Money Management/Trading Plan' },
		{ id: '531', label: 'Indicators' },
		{ id: '3260', label: 'Options' },
		{ id: '469', label: 'foundation' },
		{ id: '527', label: 'Fundamentals' },
		{ id: '522', label: 'Simpler Tech' },
		{ id: '2929', label: 'Charting/Indicators/Tools' },
		{ id: '530', label: 'Charting' },
		{ id: '3515', label: 'Drama Free Daytrades' },
		{ id: '3516', label: 'Quick Hits Daytrades' },
		{ id: '537', label: 'Psychology' },
		{ id: '775', label: 'Trading Platform' },
		{ id: '3055', label: 'Calls' },
		{ id: '447', label: 'ThinkorSwim' },
		{ id: '446', label: 'TradeStation' },
		{ id: '776', label: 'Charting Software' },
		{ id: '772', label: 'Trading Computer' },
		{ id: '3057', label: 'Calls Puts Credit Spreads' },
		{ id: '3056', label: 'Puts' },
		{ id: '3514', label: 'Profit Recycling' },
		{ id: '791', label: 'Trade Strategies' },
		{ id: '774', label: 'Website Support' },
		{ id: '2927', label: 'Options Strategies (Level 2 & 3)' },
		{ id: '457', label: 'Crypto' },
		{ id: '2931', label: 'Fibonacci & Options Trading' },
		{ id: '2928', label: 'Pricing/Volatility' },
		{ id: '459', label: 'Crypto Indicators & Trading' },
		{ id: '771', label: 'Browser Support' },
		{ id: '2930', label: 'Earnings & Options Expiration' }
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

<TradingRoomHeader 
	roomName="Day Trading Room" 
	startHereUrl="/dashboard/day-trading-room/start-here" 
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filter Form - matches WordPress exactly -->
		<form action="https://www.simplertrading.com/cms/wp-admin/admin-ajax.php" method="POST" id="term_filter">
			<div class="reset_filter">
				<input 
					type="radio" 
					id="0" 
					value="0" 
					name="categoryfilter"
					checked={activeFilter === 'all'}
					onchange={() => filterResources('all')}
				/>
				<label for="0">
					<svg aria-hidden="true" focusable="false" data-prefix="fas" data-icon="undo" class="svg-inline--fa fa-undo fa-w-16" role="img" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
						<path fill="currentColor" d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"></path>
					</svg>
				</label>
			</div>
			<div class="filter_btn"><input type="radio" id="529" value="529" name="categoryfilter" onchange={() => filterResources('529')}><label for="529">Trade Setups &amp; Strategies</label></div><div class="filter_btn"><input type="radio" id="528" value="528" name="categoryfilter" onchange={() => filterResources('528')}><label for="528">Methodology</label></div><div class="filter_btn"><input type="radio" id="329" value="329" name="categoryfilter" onchange={() => filterResources('329')}><label for="329">Member Webinar</label></div><div class="filter_btn"><input type="radio" id="2932" value="2932" name="categoryfilter" onchange={() => filterResources('2932')}><label for="2932">Trade &amp; Money Management/Trading Plan</label></div><div class="filter_btn"><input type="radio" id="531" value="531" name="categoryfilter" onchange={() => filterResources('531')}><label for="531">Indicators</label></div><div class="filter_btn"><input type="radio" id="3260" value="3260" name="categoryfilter" onchange={() => filterResources('3260')}><label for="3260">Options</label></div><div class="filter_btn"><input type="radio" id="469" value="469" name="categoryfilter" onchange={() => filterResources('469')}><label for="469">foundation</label></div><div class="filter_btn"><input type="radio" id="527" value="527" name="categoryfilter" onchange={() => filterResources('527')}><label for="527">Fundamentals</label></div><div class="filter_btn"><input type="radio" id="522" value="522" name="categoryfilter" onchange={() => filterResources('522')}><label for="522">Simpler Tech</label></div><div class="filter_btn"><input type="radio" id="2929" value="2929" name="categoryfilter" onchange={() => filterResources('2929')}><label for="2929">Charting/Indicators/Tools</label></div><div class="filter_btn"><input type="radio" id="530" value="530" name="categoryfilter" onchange={() => filterResources('530')}><label for="530">Charting</label></div><div class="filter_btn"><input type="radio" id="3515" value="3515" name="categoryfilter" onchange={() => filterResources('3515')}><label for="3515">Drama Free Daytrades</label></div><div class="filter_btn"><input type="radio" id="3516" value="3516" name="categoryfilter" onchange={() => filterResources('3516')}><label for="3516">Quick Hits Daytrades</label></div><div class="filter_btn"><input type="radio" id="537" value="537" name="categoryfilter" onchange={() => filterResources('537')}><label for="537">Psychology</label></div><div class="filter_btn"><input type="radio" id="775" value="775" name="categoryfilter" onchange={() => filterResources('775')}><label for="775">Trading Platform</label></div><div class="filter_btn"><input type="radio" id="3055" value="3055" name="categoryfilter" onchange={() => filterResources('3055')}><label for="3055">Calls</label></div><div class="filter_btn"><input type="radio" id="447" value="447" name="categoryfilter" onchange={() => filterResources('447')}><label for="447">ThinkorSwim</label></div><div class="filter_btn"><input type="radio" id="446" value="446" name="categoryfilter" onchange={() => filterResources('446')}><label for="446">TradeStation</label></div><div class="filter_btn"><input type="radio" id="776" value="776" name="categoryfilter" onchange={() => filterResources('776')}><label for="776">Charting Software</label></div><div class="filter_btn"><input type="radio" id="772" value="772" name="categoryfilter" onchange={() => filterResources('772')}><label for="772">Trading Computer</label></div><div class="filter_btn"><input type="radio" id="3057" value="3057" name="categoryfilter" onchange={() => filterResources('3057')}><label for="3057">Calls Puts Credit Spreads</label></div><div class="filter_btn"><input type="radio" id="3056" value="3056" name="categoryfilter" onchange={() => filterResources('3056')}><label for="3056">Puts</label></div><div class="filter_btn"><input type="radio" id="3514" value="3514" name="categoryfilter" onchange={() => filterResources('3514')}><label for="3514">Profit Recycling</label></div><div class="filter_btn"><input type="radio" id="791" value="791" name="categoryfilter" onchange={() => filterResources('791')}><label for="791">Trade Strategies</label></div><div class="filter_btn"><input type="radio" id="774" value="774" name="categoryfilter" onchange={() => filterResources('774')}><label for="774">Website Support</label></div><div class="filter_btn"><input type="radio" id="2927" value="2927" name="categoryfilter" onchange={() => filterResources('2927')}><label for="2927">Options Strategies (Level 2 &amp; 3)</label></div><div class="filter_btn"><input type="radio" id="457" value="457" name="categoryfilter" onchange={() => filterResources('457')}><label for="457">Crypto</label></div><div class="filter_btn"><input type="radio" id="2931" value="2931" name="categoryfilter" onchange={() => filterResources('2931')}><label for="2931">Fibonacci &amp; Options Trading</label></div><div class="filter_btn"><input type="radio" id="2928" value="2928" name="categoryfilter" onchange={() => filterResources('2928')}><label for="2928">Pricing/Volatility</label></div><div class="filter_btn"><input type="radio" id="459" value="459" name="categoryfilter" onchange={() => filterResources('459')}><label for="459">Crypto Indicators &amp; Trading</label></div><div class="filter_btn"><input type="radio" id="771" value="771" name="categoryfilter" onchange={() => filterResources('771')}><label for="771">Browser Support</label></div><div class="filter_btn"><input type="radio" id="2930" value="2930" name="categoryfilter" onchange={() => filterResources('2930')}><label for="2930">Earnings &amp; Options Expiration</label></div>
			<input type="hidden" name="page_id" value="402087">
			<input type="hidden" name="pagination_base" value="https://www.simplertrading.com/dashboard/mastering-the-trade/learning-center/page/%#%">
			
			<button class="apply_filter">Apply filter</button>
			<input type="hidden" name="action" value="myfilter">
		</form>

		<!-- Section Title - matches WordPress exactly -->
		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Learning Center<span> | </span><span>Overview</span></h2>
			<p></p>
		</section>

		<!-- Learning Resources Grid -->
		<div id="response">
			<div class="article-cards row flex-grid">
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
						
						<a href="/learning-center/{resource.slug}" class="btn btn-tiny btn-default watch-now-btn">
							Watch Now
						</a>
					</article>
				</div>
			{/each}
			</div>
		</div>

		<!-- Pagination -->
		{#if totalPages > 1}
			<div class="fl-builder-pagination">
				<ul class="page-numbers">
					{#if currentPage > 1}
						<li>
							<button 
								class="page-numbers" 
								onclick={() => goToPage(currentPage - 1)}
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
									onclick={() => goToPage(Number(pageNum))}
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
								onclick={() => goToPage(currentPage + 1)}
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
	/* Section Title */
	.dashboard__content-section {
		margin-bottom: 20px;
	}

	.section-title {
		font-family: 'Open Sans Condensed', 'Open Sans', sans-serif;
		font-size: 24px;
		font-weight: 700;
		color: #333;
		margin: 0 0 10px;
	}

	.section-title span {
		color: #666;
		font-weight: 400;
	}

	/* Term Filter Form - Exact WordPress CSS from dashboard.8f78208b.css */
	#term_filter {
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		overflow-x: scroll;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
		background: #fff;
		padding: 20px;
		-webkit-box-shadow: 0 3px 6px #00000029;
		box-shadow: 0 3px 6px #00000029;
	}

	#term_filter .apply_filter {
		display: none;
	}

	/* Reset Filter Button */
	#term_filter .reset_filter {
		margin-right: 10px;
	}

	#term_filter .reset_filter :global(svg) {
		min-width: 20px;
	}

	#term_filter .reset_filter label {
		border-radius: 25px;
		cursor: pointer;
		padding: 13px 32px;
		border: 2px solid;
		display: -webkit-flex;
		display: -ms-flexbox;
		display: flex;
		margin-bottom: 0;
		-webkit-align-items: center;
		-ms-flex-align: center;
		align-items: center;
	}

	#term_filter .reset_filter input {
		display: none;
	}

	/* Filter Buttons */
	#term_filter .filter_btn {
		margin-right: 10px;
	}

	#term_filter .filter_btn input {
		display: none;
	}

	#term_filter .filter_btn.checked label,
	#term_filter .filter_btn input:checked + label {
		background: #333;
		color: #fff;
	}

	#term_filter .filter_btn label {
		cursor: pointer;
		padding: 11px 32px;
		border-radius: 25px;
		border: 2px solid;
		white-space: nowrap;
		font-weight: 700;
		color: #666;
		display: block;
		margin-bottom: 0;
	}

	/* Grid Layout */
	.article-cards {
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
		#term_filter {
			padding: 15px;
			gap: 6px;
		}
		
		.filter_btn label {
			padding: 6px 10px;
			font-size: 11px;
		}

		.reset_filter label {
			width: 32px;
			height: 32px;
		}
	}

	@media (max-width: 575px) {
		.col-xs-12 {
			width: 100%;
		}
		
		.article-cards {
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
