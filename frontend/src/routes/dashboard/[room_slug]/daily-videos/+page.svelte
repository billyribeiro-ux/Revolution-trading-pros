<!--
	URL: /dashboard/[room_slug]/daily-videos
	
	Premium Daily Videos Page - Multi-Room Support
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Daily video analysis and market commentary with dynamic room selection.
	Includes pagination matching WordPress implementation.
	
	@version 3.0.0 - January 2026 - Multi-room support
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
	const roomSlug = $derived(data.roomSlug);
	const roomName = $derived(data.roomName);

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
	<title>Premium Daily Videos | {roomName} | Revolution Trading Pros</title>
	<meta name="description" content="Access daily video analysis and market commentary from our expert traders." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<!-- Dashboard Header -->
<header class="dashboard__header">
	<div class="dashboard__header-left">
		<h1 class="dashboard__page-title">{roomName} Dashboard</h1>
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
	</div>
</header>

<section class="dashboard__content-section">
	<h2 class="section-title">{roomName} Premium Daily Videos</h2>
	<p></p>
	<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <span class="facetwp-counts">{(currentPage - 1) * itemsPerPage + 1}-{Math.min(currentPage * itemsPerPage, totalItems)} of {totalItems}</span>
				</div>
				<div class="dashboard-filters__search">
					<input 
						type="text"
						id="video-search"
						name="video-search"
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
				{#if displayedVideos.length === 0}
					<div class="empty-state">
						<p>No videos found. Please check back later.</p>
					</div>
				{:else}
				<div class="card-grid flex-grid row">
			{#each displayedVideos as video (video.id)}
				<article class="card-grid-spacer flex-grid-item col-xs-12 col-sm-6 col-md-6 col-lg-4">
					<div class="card flex-grid-panel">
						<figure class="card-media card-media--video">
							<a 
								href="/dashboard/{roomSlug}/video/{video.slug}" 
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
								<a href="/dashboard/{roomSlug}/video/{video.slug}">
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
							<a class="btn btn-tiny btn-default" href="/dashboard/{roomSlug}/video/{video.slug}">
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
							{#if pageNum === '...'}
								<span class="facetwp-page dots">…</span>
							{:else if pageNum === currentPage}
								<span class="facetwp-page active">{pageNum}</span>
							{:else}
								<button 
									type="button"
									class="facetwp-page" 
									onclick={() => goToPage(Number(pageNum))}
								>
									{pageNum}
								</button>
							{/if}
						{/each}
					</div>
				</div>
				{/if}
			</div>
</section>

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

	/* ═══════════════════════════════════════════════════════════════════════════
	 * DASHBOARD CONTENT SECTION
	 * ═══════════════════════════════════════════════════════════════════════════ */

	.dashboard__content-section {
		margin-bottom: 3rem;
		padding: 2rem 1.5rem;
	}

	.section-title {
		font-size: 1.75rem;
		font-weight: 600;
		margin-bottom: 1.5rem;
		color: #1a1a1a;
	}

	/* Filters */
	.dashboard-filters {
		display: flex;
		justify-content: space-between;
		align-items: center;
		margin-bottom: 2rem;
		gap: 1rem;
		flex-wrap: wrap;
	}

	.dashboard-filters__count {
		font-size: 0.95rem;
		color: #666;
	}

	.facetwp-counts {
		font-weight: 600;
		color: #333;
	}

	.dashboard-filters__search {
		display: flex;
		gap: 0;
		max-width: 300px;
		flex: 1;
	}

	.facetwp-autocomplete {
		flex: 1;
		padding: 8px 12px;
		font-size: 14px;
		border: 1px solid #ddd;
		border-radius: 4px 0 0 4px;
		outline: none;
		font-family: 'Montserrat', sans-serif;
	}

	.facetwp-autocomplete:focus {
		border-color: #F69532;
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

	/* Card Grid */
	.card-grid {
		display: flex;
		flex-wrap: wrap;
		margin: 0 -15px;
	}

	.card-grid-spacer {
		padding: 0 15px;
		margin-bottom: 30px;
	}

	.card {
		background: #fff;
		border: 1px solid #e5e5e5;
		border-radius: 4px;
		overflow: hidden;
		transition: all 0.3s ease;
		display: flex;
		flex-direction: column;
		height: 100%;
	}

	.card:hover {
		box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
		transform: translateY(-2px);
	}

	/* Card Media */
	.card-media {
		position: relative;
		margin: 0;
		overflow: hidden;
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
		z-index: 2;
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
		z-index: 3;
	}

	.card-image {
		display: block;
		position: relative;
		padding-top: 56.25%;
		background-size: cover;
		background-position: center;
		transition: transform 0.3s ease;
	}

	.card:hover .card-image {
		transform: scale(1.05);
	}

	.card-image img {
		position: absolute;
		top: 0;
		left: 0;
		width: 100%;
		height: 100%;
		opacity: 0;
	}

	/* Card Body */
	.card-body {
		padding: 1.25rem;
		flex: 1;
	}

	.card-title {
		margin: 0 0 0.75rem;
		font-size: 1.1rem;
		line-height: 1.4;
	}

	.card-title a {
		color: #1a1a1a;
		text-decoration: none;
		transition: color 0.2s ease;
	}

	.card-title a:hover {
		color: #F69532;
	}

	.article-card__meta {
		color: #666;
		font-size: 0.85rem;
	}

	.card-description {
		margin-top: 0.75rem;
		color: #555;
		font-size: 0.9rem;
		line-height: 1.5;
	}

	.u--squash {
		display: -webkit-box;
		-webkit-line-clamp: 3;
		line-clamp: 3;
		-webkit-box-orient: vertical;
		overflow: hidden;
	}

	/* Card Footer */
	.card-footer {
		padding: 0 1.25rem 1.25rem;
		text-align: center;
	}

	.btn {
		display: inline-block;
		padding: 10px 24px;
		font-size: 14px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
		font-family: 'Montserrat', sans-serif;
	}

	.btn-tiny {
		padding: 8px 20px;
		font-size: 13px;
	}

	.btn-default {
		background: #F69532;
		color: #fff;
	}

	.btn-default:hover {
		background: #dc7309;
		color: #fff;
	}

	/* Pagination */
	.facetwp-pagination {
		margin-top: 3rem;
		padding-top: 2rem;
		border-top: 1px solid #e5e5e5;
	}

	.facetwp-pager {
		display: flex;
		justify-content: center;
		align-items: center;
		gap: 0.5rem;
		flex-wrap: wrap;
	}

	.facetwp-page {
		display: inline-flex;
		align-items: center;
		justify-content: center;
		min-width: 40px;
		height: 40px;
		padding: 0 12px;
		font-size: 14px;
		font-weight: 500;
		color: #333;
		background: #fff;
		border: 1px solid #ddd;
		border-radius: 4px;
		text-decoration: none;
		transition: all 0.2s ease;
		cursor: pointer;
	}

	.facetwp-page:hover {
		background: #f5f5f5;
		border-color: #F69532;
		color: #F69532;
	}

	.facetwp-page.active {
		background: #F69532;
		border-color: #F69532;
		color: #fff;
		cursor: default;
	}

	.facetwp-page.dots {
		border: none;
		cursor: default;
		background: transparent;
	}

	.facetwp-page.dots:hover {
		background: transparent;
		border: none;
		color: #333;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 4rem 2rem;
		color: #666;
	}

	.empty-state p {
		font-size: 1.1rem;
		margin: 0;
	}

	/* Responsive Grid */
	@media (min-width: 768px) {
		.col-sm-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 992px) {
		.col-md-6 {
			flex: 0 0 50%;
			max-width: 50%;
		}
	}

	@media (min-width: 1200px) {
		.col-lg-4 {
			flex: 0 0 33.333333%;
			max-width: 33.333333%;
		}
	}

	.col-xs-12 {
		flex: 0 0 100%;
		max-width: 100%;
	}

	/* Mobile-first: smaller padding by default, larger on md+ */
	@media (min-width: 768px) {
		.section-title {
			font-size: 1.5rem;
		}

		.dashboard-filters {
			flex-direction: column;
			align-items: stretch;
		}

		.dashboard-filters__search {
			max-width: 100%;
		}
	}
</style>
