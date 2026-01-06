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
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Dropdown state
	let isDropdownOpen = $state(false);

	// Trading rooms for dropdown
	const tradingRooms = [
		{
			name: 'Day Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'chart-line'
		},
		{
			name: 'Swing Trading Room',
			href: '#', // TODO: Provide URL
			icon: 'trending-up'
		},
		{
			name: 'Small Accounts Mentorship',
			href: '#', // TODO: Provide URL
			icon: 'dollar-sign'
		}
	];

	function toggleDropdown(event: Event): void {
		event.stopPropagation();
		isDropdownOpen = !isDropdownOpen;
	}

	function closeDropdown(): void {
		isDropdownOpen = false;
	}

	// Close dropdown when clicking outside
	$effect(() => {
		if (isDropdownOpen && typeof window !== 'undefined') {
			const handleClickOutside = (e: MouseEvent) => {
				const target = e.target as HTMLElement;
				if (!target.closest('.dropdown')) {
					closeDropdown();
				}
			};
			const handleEscape = (e: KeyboardEvent) => {
				if (e.key === 'Escape') {
					closeDropdown();
				}
			};
			document.addEventListener('click', handleClickOutside);
			document.addEventListener('keydown', handleEscape);
			return () => {
				document.removeEventListener('click', handleClickOutside);
				document.removeEventListener('keydown', handleEscape);
			};
		}
		return undefined;
	});

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

	// Category options matching WordPress exactly
	const categories = [
		{ id: 'trade-setups', label: 'Trade Setups & Strategies' },
		{ id: 'methodology', label: 'Methodology' },
		{ id: 'member-webinar', label: 'Member Webinar' },
		{ id: 'trade-management', label: 'Trade & Money Management/Trading Plan' },
		{ id: 'indicators', label: 'Indicators' },
		{ id: 'options', label: 'Options' },
		{ id: 'foundation', label: 'Foundation' },
		{ id: 'fundamentals', label: 'Fundamentals' },
		{ id: 'simpler-tech', label: 'Simpler Tech' },
		{ id: 'charting-indicators-tools', label: 'Charting/Indicators/Tools' },
		{ id: 'charting', label: 'Charting' },
		{ id: 'drama-free-daytrades', label: 'Drama Free Daytrades' },
		{ id: 'quick-hits-daytrades', label: 'Quick Hits Daytrades' },
		{ id: 'psychology', label: 'Psychology' },
		{ id: 'trading-platform', label: 'Trading Platform' },
		{ id: 'calls', label: 'Calls' },
		{ id: 'thinkorswim', label: 'ThinkorSwim' },
		{ id: 'tradestation', label: 'TradeStation' },
		{ id: 'charting-software', label: 'Charting Software' },
		{ id: 'trading-computer', label: 'Trading Computer' },
		{ id: 'calls-puts-credit-spreads', label: 'Calls Puts Credit Spreads' },
		{ id: 'puts', label: 'Puts' },
		{ id: 'profit-recycling', label: 'Profit Recycling' },
		{ id: 'trade-strategies', label: 'Trade Strategies' },
		{ id: 'website-support', label: 'Website Support' },
		{ id: 'options-strategies', label: 'Options Strategies (Level 2 & 3)' },
		{ id: 'crypto', label: 'Crypto' },
		{ id: 'fibonacci-options', label: 'Fibonacci & Options Trading' },
		{ id: 'pricing-volatility', label: 'Pricing/Volatility' },
		{ id: 'crypto-indicators', label: 'Crypto Indicators & Trading' },
		{ id: 'browser-support', label: 'Browser Support' },
		{ id: 'earnings-expiration', label: 'Earnings & Options Expiration' }
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

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">Day Trading Room Dashboard</h1>
		<a href="/dashboard/day-trading-room/start-here" class="btn btn-xs btn-default">
			New? Start Here
		</a>
	</div>
	<div class="dashboard__header-right">
		<div class="dropdown" class:is-open={isDropdownOpen}>
			<button
				class="btn btn-orange btn-tradingroom"
				onclick={toggleDropdown}
				aria-expanded={isDropdownOpen}
				aria-haspopup="true"
				type="button"
			>
				<strong>Enter the Trading Room</strong>
				<span class="dropdown-arrow">
					<RtpIcon name="chevron-down" size={14} />
				</span>
			</button>

			{#if isDropdownOpen}
				<div class="dropdown-menu" role="menu">
					{#each tradingRooms as room}
						<a 
							href={room.href} 
							class="dropdown-item" 
							onclick={closeDropdown}
							role="menuitem"
						>
							<span class="dropdown-item__icon">
								<RtpIcon name={room.icon} size={20} />
							</span>
							<span class="dropdown-item__text">{room.name}</span>
						</a>
					{/each}
				</div>
			{/if}
		</div>

		<!-- Trading Room Rules - Legal Compliance -->
		<div class="trading-room-rules">
			<a
				href="/trading-room-rules.pdf"
				target="_blank"
				rel="noopener noreferrer"
				class="trading-room-rules__link"
			>
				Trading Room Rules
			</a>
			<p class="trading-room-rules__disclaimer">
				By logging into any of our Live Trading Rooms, You are agreeing to our Rules of the Room.
			</p>
		</div>
	</div>
</header>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<!-- Category Filter Form - matches WordPress exactly -->
		<form action="#" method="POST" id="term_filter" onsubmit={(e) => e.preventDefault()}>
			<!-- Reset Filter Button -->
			<div class="reset_filter">
				<input 
					type="radio" 
					id="filter-reset" 
					value="all" 
					name="categoryfilter"
					checked={activeFilter === 'all'}
					onchange={() => filterResources('all')}
				/>
				<label for="filter-reset" title="Reset Filter">
					<svg aria-hidden="true" focusable="false" class="reset-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
						<path fill="currentColor" d="M212.333 224.333H12c-6.627 0-12-5.373-12-12V12C0 5.373 5.373 0 12 0h48c6.627 0 12 5.373 12 12v78.112C117.773 39.279 184.26 7.47 258.175 8.007c136.906.994 246.448 111.623 246.157 248.532C504.041 393.258 393.12 504 256.333 504c-64.089 0-122.496-24.313-166.51-64.215-5.099-4.622-5.334-12.554-.467-17.42l33.967-33.967c4.474-4.474 11.662-4.717 16.401-.525C170.76 415.336 211.58 432 256.333 432c97.268 0 176-78.716 176-176 0-97.267-78.716-176-176-176-58.496 0-110.28 28.476-142.274 72.333h98.274c6.627 0 12 5.373 12 12v48c0 6.627-5.373 12-12 12z"></path>
					</svg>
				</label>
			</div>

			<!-- Category Filter Buttons -->
			{#each categories as category}
				<div class="filter_btn" class:active={activeFilter === category.id}>
					<input 
						type="radio" 
						id="filter-{category.id}" 
						value={category.id} 
						name="categoryfilter"
						checked={activeFilter === category.id}
						onchange={() => filterResources(category.id)}
					/>
					<label for="filter-{category.id}">{category.label}</label>
				</div>
			{/each}
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
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - WordPress Exact Match
	 * Source: frontend/Implementation/learning-center
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__header {
		display: flex;
		flex-wrap: wrap;
		align-items: center;
		background-color: #fff;
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
		display: flex;
		align-items: center;
		justify-content: flex-start;
		flex: 1;
		gap: 15px;
	}

	.dashboard__page-title {
		margin: 0;
		color: #333;
		font-size: 36px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.dashboard__header-right {
		display: flex;
		flex-direction: column;
		align-items: flex-end;
		margin-top: 10px;
	}

	@media (min-width: 820px) {
		.dashboard__header-right {
			flex-direction: column;
			align-items: flex-end;
			gap: 0;
			margin-top: 0;
		}
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * BUTTONS - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.15s ease-in-out;
		cursor: pointer;
		border: none;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
	}

	.btn-xs {
		padding: 6px 12px;
		font-size: 12px;
	}

	.btn-default {
		background-color: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background-color: #0f2d41;
		border-color: #0f2d41;
	}

	.btn-orange {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		gap: 8px;
		padding: 10px 20px;
		background-color: #f69532;
		color: #fff;
		font-size: 14px;
		font-weight: 600;
		text-decoration: none;
		border: none;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.15s ease-in-out;
	}

	.btn-orange:hover {
		background-color: #dc7309;
	}

	.btn-orange strong {
		font-weight: 700;
	}

	.btn-tradingroom {
		text-transform: none;
		width: 280px;
		padding: 12px 18px;
	}

	.dropdown-arrow {
		font-size: 10px;
		transition: transform 0.15s ease-in-out;
		display: flex;
		align-items: center;
	}

	.dropdown.is-open .dropdown-arrow {
		transform: rotate(180deg);
	}

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DROPDOWN MENU - WordPress Exact Match
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dropdown {
		position: relative;
		display: inline-block;
	}

	.dropdown-menu {
		position: absolute;
		top: 100%;
		right: 0;
		padding: 15px;
		min-width: 260px;
		max-width: 280px;
		margin: 5px 0 0;
		font-size: 14px;
		background-color: #ffffff;
		border: none;
		border-radius: 5px;
		box-shadow: 0 10px 30px rgba(0, 0, 0, 0.15);
		z-index: 1000;
		opacity: 1;
		visibility: visible;
		transform: translateY(0);
		transition: all 0.15s ease-in-out;
	}

	.dropdown:not(.is-open) .dropdown-menu {
		opacity: 0;
		visibility: hidden;
		transform: translateY(-5px);
		pointer-events: none;
	}

	.dropdown-item {
		display: flex;
		align-items: center;
		gap: 6px;
		padding: 15px 20px;
		color: #666;
		font-size: 14px;
		font-weight: 400;
		text-decoration: none;
		transition: background-color 0.15s ease-in-out;
		border-radius: 5px;
		white-space: nowrap;
	}

	.dropdown-item:hover {
		background-color: #f4f4f4;
	}

	.dropdown-item__icon {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-shrink: 0;
		color: #143E59;
	}

	.dropdown-item__text {
		flex: 1;
	}

	/* Trading Room Rules - Legal Compliance */
	.trading-room-rules {
		text-align: center;
		margin-top: 10px;
		width: 100%;
		max-width: 300px;
		margin-left: auto;
		margin-right: auto;
	}

	.trading-room-rules__link {
		display: block;
		margin-bottom: 8px;
		font-size: 18px;
		font-weight: 700;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #1e73be;
		text-decoration: none;
		transition: color 0.15s ease-in-out;
		text-align: center;
	}

	.trading-room-rules__link:hover {
		color: #0984ae;
		text-decoration: underline;
	}

	.trading-room-rules__disclaimer {
		margin: 0;
		font-size: 13px;
		font-weight: 400;
		font-family: var(--font-heading), 'Montserrat', sans-serif;
		color: #666;
		line-height: 1.4;
		text-align: center;
	}

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

	/* Term Filter Form - matches WordPress exactly */
	#term_filter {
		display: flex;
		flex-wrap: wrap;
		gap: 8px;
		margin-bottom: 25px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 4px;
		align-items: center;
	}

	/* Reset Filter Button */
	.reset_filter {
		display: inline-flex;
	}

	.reset_filter input[type="radio"] {
		display: none;
	}

	.reset_filter label {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		width: 36px;
		height: 36px;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		color: #666;
	}

	.reset_filter label:hover {
		background: #e9e9e9;
		border-color: #ccc;
	}

	.reset_filter input[type="radio"]:checked + label {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	.reset-icon {
		width: 14px;
		height: 14px;
	}

	/* Filter Buttons */
	.filter_btn {
		display: inline-flex;
	}

	.filter_btn input[type="radio"] {
		display: none;
	}

	.filter_btn label {
		padding: 8px 14px;
		border: 1px solid #ddd;
		background: #fff;
		color: #333;
		font-size: 13px;
		font-weight: 500;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		white-space: nowrap;
	}

	.filter_btn label:hover {
		background: #e9e9e9;
		border-color: #ccc;
	}

	.filter_btn input[type="radio"]:checked + label,
	.filter_btn.active label {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
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
