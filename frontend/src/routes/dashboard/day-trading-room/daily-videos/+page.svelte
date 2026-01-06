<!--
	URL: /dashboard/day-trading-room/daily-videos
	
	Premium Daily Videos Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Daily video analysis and market commentary for Day Trading Room members.
	Includes pagination matching WordPress implementation.
	
	@version 2.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import type { DailyVideo, PageData } from './+page.server';
	import RtpIcon from '$lib/components/icons/RtpIcon.svelte';

	// Server-loaded data
	let { data }: { data: PageData } = $props();

	// Reactive state from server data
	const displayedVideos = $derived(data.videos || []);
	const currentPage = $derived(data.pagination?.page || 1);
	const totalPages = $derived(data.pagination?.totalPages || 1);
	const itemsPerPage = $derived(data.pagination?.perPage || 12);
	const totalItems = $derived(data.pagination?.total || 0);

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

	// Search state
	let searchQuery = $state('');

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

	function handleSearch(event: Event) {
		const target = event.target as HTMLInputElement;
		searchQuery = target.value;
	}

	function submitSearch() {
		const url = new URL(window.location.href);
		if (searchQuery) {
			url.searchParams.set('search', searchQuery);
		} else {
			url.searchParams.delete('search');
		}
		url.searchParams.set('page', '1');
		goto(url.toString(), { invalidateAll: true });
	}

	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			const url = new URL(window.location.href);
			url.searchParams.set('page', pageNum.toString());
			goto(url.toString(), { invalidateAll: true });
			// Scroll to top
			window.scrollTo({ top: 0, behavior: 'smooth' });
		}
	}

	function getPaginationRange(): (number | string)[] {
		const range: (number | string)[] = [];
		
		// Always show first page
		range.push(1);
		
		if (currentPage > 3) {
			range.push('...');
		}
		
		// Show pages around current page
		for (let i = Math.max(2, currentPage - 1); i <= Math.min(totalPages - 1, currentPage + 1); i++) {
			if (!range.includes(i)) {
				range.push(i);
			}
		}
		
		if (currentPage < totalPages - 2) {
			range.push('...');
		}
		
		// Always show last page
		if (totalPages > 1 && !range.includes(totalPages)) {
			range.push(totalPages);
		}
		
		return range;
	}
</script>

<svelte:head>
	<title>Premium Daily Videos | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access daily video analysis and market commentary from our expert traders." />
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
		<section class="dashboard__content-section">
			<h2 class="section-title">Day Trading Room Premium Daily Videos</h2>
			<p></p>
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{(currentPage - 1) * itemsPerPage + 1}-{Math.min(currentPage * itemsPerPage, totalItems)} of {totalItems}</span>
				</div>
				<div class="dashboard-filters__search">
					<input 
						type="text" 
						class="facetwp-autocomplete" 
						placeholder="Search" 
						value={searchQuery}
						oninput={handleSearch}
						aria-label="Search premium daily videos"
						autocomplete="off"
					/>
					<button 
						type="button" 
						class="facetwp-autocomplete-update"
						aria-label="Search"
					>
						<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
							<circle cx="11" cy="11" r="8"></circle>
							<path d="m21 21-4.35-4.35"></path>
						</svg>
					</button>
				</div>
			</div>
			<div id="products-list" class="facetwp-template">

				<div class="card-grid flex-grid row">
			{#each displayedVideos as video (video.id)}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a 
								href="/dashboard/day-trading-room/video/{video.slug}" 
								class="card-image" 
								style="background-image: url({video.thumbnail});"
							>
								<img 
									class="default-background" 
									width="325" 
									height="183" 
									alt={video.title}
									loading="lazy"
								/>
							</a>
						</figure>
						
						<section class="card-body">
							<h4 class="h5 card-title">
								<a href="/dashboard/day-trading-room/video/{video.slug}">
									{video.title}
								</a>
							</h4>
							<span class="article-card__meta">
								<small>{video.date} with {video.trader}</small>
							</span>
							<br>
							<div class="card-description">
								<div class="u--hide-read-more u--squash">
									<p>{video.excerpt}</p>
								</div>
							</div>
						</section>
						
						<footer class="card-footer">
							<a class="btn btn-tiny btn-default" href="/dashboard/day-trading-room/video/{video.slug}">
								Watch Now
							</a>
						</footer>
					</div>
				</article>
			{/each}
				</div>
				<div class="facetwp-pagination">
					<div class="facetwp-pager">
						{#each getPaginationRange() as pageNum}
							{#if pageNum === currentPage}
								<span class="facetwp-page active" data-page="{pageNum}" aria-current="page">{pageNum}</span>
							{:else if pageNum === totalPages && pageNum !== currentPage}
								<a 
									href="#page-{pageNum}"
									class="facetwp-page last-page" 
									data-page="{pageNum}"
									onclick={(e) => { e.preventDefault(); goToPage(Number(pageNum)); }}
									aria-label="Go to last page {pageNum}"
								>
									<span class="fa fa-angle-double-right"></span>
								</a>
							{:else if pageNum !== '...'}
								<a 
									href="#page-{pageNum}"
									class="facetwp-page" 
									data-page="{pageNum}"
									onclick={(e) => { e.preventDefault(); goToPage(Number(pageNum)); }}
									aria-label="Go to page {pageNum}"
								>
									{pageNum}
								</a>
							{/if}
						{/each}
					</div>
				</div>
			</div>
		</section>
	</div>
</div>

<style>
	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD HEADER - WordPress Exact Match
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
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
	 * Source: DASHBOARD_DESIGN_SPECIFICATIONS.md
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

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	/* Dashboard Filters */
	.dashboard-filters {
		margin-bottom: 30px;
		padding: 20px;
		background: #f5f5f5;
		border-radius: 4px;
		display: flex;
		justify-content: space-between;
		align-items: center;
		gap: 20px;
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: #666;
		display: flex;
		align-items: center;
		gap: 5px;
		flex-shrink: 0;
	}

	.dashboard-filters__search {
		flex: 0 0 auto;
		min-width: 250px;
		max-width: 320px;
		display: flex;
		gap: 0;
	}
	
	.facetwp-autocomplete {
		flex: 1;
		padding: 8px 12px;
		font-size: 13px;
		border: 1px solid #ddd;
		border-radius: 4px 0 0 4px;
		background: #fff;
		color: #333;
		font-family: 'Montserrat', sans-serif;
		transition: border-color 0.2s ease;
	}
	
	.facetwp-autocomplete:focus {
		outline: none;
		border-color: #F69532;
		box-shadow: 0 0 0 2px rgba(246, 149, 50, 0.1);
	}
	
	.facetwp-autocomplete::placeholder {
		color: #999;
	}

	.facetwp-autocomplete-update {
		padding: 8px 12px;
		font-size: 14px;
		border: 1px solid #ddd;
		border-left: none;
		border-radius: 0 4px 4px 0;
		background: #F69532;
		color: #fff;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Montserrat', sans-serif;
		display: flex;
		align-items: center;
		justify-content: center;
	}

	.facetwp-autocomplete-update:hover {
		background: #dc7309;
		color: #fff;
	}

	.facetwp-counts {
		font-weight: 600;
		color: #333;
		font-family: 'Montserrat', sans-serif;
	}

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px 30px;
	}

	.flex-grid-item {
		display: flex;
	}

	.card {
		display: flex;
		flex-direction: column;
		width: 100%;
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.3s ease;
	}

	.card:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
	}

	.card-media {
		position: relative;
		width: 100%;
		padding-top: 56.25%; /* 16:9 aspect ratio */
		overflow: hidden;
		margin: 0;
	}

	.card-media--video::after {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 0;
		height: 0;
		border-style: solid;
		border-width: 20px 0 20px 35px;
		border-color: transparent transparent transparent rgba(255, 255, 255, 0.95);
		pointer-events: none;
		filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
	}

	.card-media--video::before {
		content: '';
		position: absolute;
		top: 50%;
		left: 50%;
		transform: translate(-50%, -50%);
		width: 70px;
		height: 70px;
		background: rgba(0, 0, 0, 0.6);
		border-radius: 50%;
		pointer-events: none;
	}

	.card-image {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		background-size: cover;
		background-position: center;
		display: block;
	}

	.card-image img {
		opacity: 0;
		width: 100%;
		height: 100%;
	}

	.card-body {
		padding: 20px;
		flex: 1;
	}

	.card-title {
		margin: 0 0 10px;
		font-size: 18px;
		font-weight: 700;
		line-height: 1.3;
	}

	.card-title a {
		color: #333;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card-title a:hover {
		color: #F69532;
	}

	.article-card__meta {
		display: block;
		margin-bottom: 10px;
	}

	.article-card__meta small {
		font-size: 12px;
		color: #999;
	}

	.card-description {
		margin-top: 10px;
		font-size: 14px;
		color: #666;
		line-height: 1.6;
	}

	.u--hide-read-more p {
		margin: 0;
	}

	.card-footer {
		padding: 15px 20px;
		border-top: 1px solid #e6e6e6;
		text-align: center;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 8px 16px;
		font-size: 14px;
		font-weight: 700;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
	}

	.btn-tiny {
		padding: 6px 12px;
		font-size: 12px;
		font-weight: 700;
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

	/* Section Title */
	.section-title {
		font-size: 28px;
		font-weight: 700;
		margin: 0 0 20px;
		color: #333;
	}

	/* Pagination */
	.facetwp-pagination {
		padding: 40px 0;
		text-align: center;
	}

	.facetwp-pager {
		display: inline-flex;
		gap: 5px;
		align-items: center;
	}

	.facetwp-page {
		display: inline-block;
		padding: 8px 12px;
		border: 1px solid #e6e6e6;
		background: #fff;
		color: #666;
		text-decoration: none;
		font-size: 14px;
		font-weight: 400;
		border-radius: 4px;
		cursor: pointer;
		transition: all 0.2s ease;
		font-family: 'Montserrat', sans-serif;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
		border-color: #ddd;
	}

	.facetwp-page.active {
		background: #F69532;
		color: #fff;
		border-color: #F69532;
		cursor: default;
		pointer-events: none;
	}

	.facetwp-page.last-page {
		font-size: 16px;
	}

	.facetwp-page.last-page .fa {
		display: inline-block;
	}

	/* Search Button SVG */
	.facetwp-autocomplete-update svg {
		display: block;
	}

	/* Responsive Grid - Mobile First */
	/* WordPress Breakpoints:
	 * xs: 0-575px (mobile)
	 * sm: 576px-767px (small tablets)
	 * md: 768px-991px (tablets)
	 * lg: 992px+ (desktop)
	 */

	/* Mobile First - Default (xs: 0-575px) */
	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	/* Small devices (sm: 576px and up) */
	@media (min-width: 576px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Medium devices (md: 768px and up) */
	@media (min-width: 768px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	/* Large devices (lg: 900px and up) - Adjusted for 14" laptops */
	@media (min-width: 900px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	/* Mobile: Stack filters vertically */
	.dashboard-filters {
		flex-direction: column;
		align-items: flex-start;
	}

	.dashboard-filters__search {
		width: 100%;
		max-width: 100%;
	}

	/* Tablet and up: Horizontal filters */
	@media (min-width: 768px) {
		.dashboard-filters {
			flex-direction: row;
			align-items: center;
		}

		.dashboard-filters__search {
			min-width: 250px;
			max-width: 320px;
		}
	}

	/* Mobile: Smaller card grid margins */
	.card-grid {
		margin: 0 -10px;
	}

	.card-grid-spacer {
		padding: 0 10px 20px;
	}

	/* Tablet and up: Larger margins */
	@media (min-width: 768px) {
		.card-grid {
			margin: 0 -15px;
		}

		.card-grid-spacer {
			padding: 0 15px 30px;
		}
	}

	/* Mobile: Smaller card title */
	.card-title {
		font-size: 16px;
	}

	/* Tablet and up: Larger card title */
	@media (min-width: 768px) {
		.card-title {
			font-size: 18px;
		}
	}

	/* Mobile: Wrap pagination */
	.facetwp-pager {
		flex-wrap: wrap;
		justify-content: center;
	}

</style>
