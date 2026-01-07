<!--
	TradingRoomArchive Component
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation - Svelte 5
	
	Reusable archive component for all trading rooms:
	- Day Trading Room
	- Swing Trading Room
	- Small Account Mentorship
	
	@props roomSlug - URL slug for the trading room
	@props roomName - Display name of the trading room
	@props videos - Array of videos from API
	@props meta - Pagination metadata
	@props search - Current search query
	@props error - Error message if any
	
	@version 1.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';

	// Video type from API
	interface VideoData {
		id: number;
		title: string;
		slug: string;
		description?: string | null;
		video_url?: string;
		thumbnail_url?: string | null;
		duration?: number | null;
		formatted_duration?: string;
		content_type?: string;
		video_date?: string;
		formatted_date?: string;
		is_published?: boolean;
		trader?: {
			id: number;
			name: string;
			slug: string;
		} | null;
		rooms?: Array<{
			id: number;
			name: string;
			slug: string;
		}>;
		tags?: string[];
		[key: string]: unknown;
	}

	// Pagination meta
	interface PaginationMeta {
		current_page: number;
		per_page: number;
		total: number;
		last_page: number;
	}

	// Props
	interface Props {
		roomSlug: string;
		roomName: string;
		videos: VideoData[];
		meta: PaginationMeta;
		search?: string;
		error?: string | null;
	}

	let { roomSlug, roomName, videos, meta, search = '', error = null }: Props = $props();

	// Archive session type (transformed from API)
	interface ArchiveSession {
		id: string;
		title: string;
		trader: string;
		date: string;
		href: string;
	}

	// Group sessions by date
	interface DateGroup {
		date: string;
		displayDate: string;
		sessions: ArchiveSession[];
	}

	// Local search state - synced with prop via $effect
	let searchQuery = $state('');
	let currentPage = $derived(meta?.current_page || 1);
	let totalPages = $derived(meta?.last_page || 1);
	let totalItems = $derived(meta?.total || 0);

	// Sync searchQuery with search prop when it changes
	$effect(() => {
		searchQuery = search || '';
	});

	// Format date to slug (MMDDYYYY)
	function dateToSlug(dateStr: string): string {
		if (!dateStr) return '';
		const date = new Date(dateStr);
		const month = String(date.getMonth() + 1).padStart(2, '0');
		const day = String(date.getDate()).padStart(2, '0');
		const year = date.getFullYear();
		return `${month}${day}${year}`;
	}

	// Transform API videos to sessions
	let allSessions = $derived<ArchiveSession[]>(
		(videos || []).map(video => ({
			id: String(video.id),
			title: video.title,
			trader: video.trader?.name || '',
			date: video.video_date?.split('T')[0] || '',
			href: `/chatroom-archive/${roomSlug}/${dateToSlug(video.video_date || '')}`
		}))
	);

	// Client-side filtering - compare against prop for server-side match
	let serverSearch = $derived(search || '');
	let filteredSessions = $derived(
		searchQuery.trim() === '' || searchQuery === serverSearch
			? allSessions
			: allSessions.filter(session => 
				session.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				session.trader.toLowerCase().includes(searchQuery.toLowerCase())
			)
	);

	// Format date for display
	function formatDate(dateStr: string): string {
		if (!dateStr) return 'Unknown Date';
		const date = new Date(dateStr);
		return date.toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' });
	}

	// Group sessions by date
	let groupedSessions = $derived(() => {
		const groups: DateGroup[] = [];
		const dateMap = new Map<string, ArchiveSession[]>();
		
		filteredSessions.forEach(session => {
			const existing = dateMap.get(session.date);
			if (existing) {
				existing.push(session);
			} else {
				dateMap.set(session.date, [session]);
			}
		});

		// Sort dates descending
		const sortedDates = Array.from(dateMap.keys()).sort((a, b) => 
			new Date(b).getTime() - new Date(a).getTime()
		);

		sortedDates.forEach(date => {
			groups.push({
				date,
				displayDate: formatDate(date),
				sessions: dateMap.get(date) || []
			});
		});

		return groups;
	});

	// Paginated groups
	let paginatedGroups = $derived(groupedSessions());

	// Handle search - navigate to update URL params
	let searchTimeout: ReturnType<typeof setTimeout>;
	function handleSearch() {
		// Debounce search
		clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			navigateWithSearch();
		}, 500);
	}

	// Handle form submit
	function handleSearchSubmit(event: Event) {
		event.preventDefault();
		clearTimeout(searchTimeout);
		navigateWithSearch();
	}

	// Clear search
	function clearSearch() {
		searchQuery = '';
		navigateWithSearch();
	}

	// Navigate with search params
	function navigateWithSearch() {
		const params = new URLSearchParams();
		if (searchQuery.trim()) {
			params.set('search', searchQuery.trim());
		}
		params.set('page', '1');
		goto(`?${params.toString()}`, { replaceState: true, noScroll: true });
	}

	// Handle page change
	function goToPage(pageNum: number) {
		if (pageNum >= 1 && pageNum <= totalPages) {
			const params = new URLSearchParams();
			if (searchQuery.trim()) {
				params.set('search', searchQuery.trim());
			}
			params.set('page', String(pageNum));
			goto(`?${params.toString()}`);
		}
	}

	// Generate page numbers for pagination
	function getPageNumbers(): (number | string)[] {
		const pages: (number | string)[] = [];
		const delta = 2;

		if (totalPages <= 7) {
			for (let i = 1; i <= totalPages; i++) {
				pages.push(i);
			}
		} else {
			pages.push(1);
			if (currentPage > delta + 2) {
				pages.push('...');
			}
			for (let i = Math.max(2, currentPage - delta); i <= Math.min(totalPages - 1, currentPage + delta); i++) {
				if (!pages.includes(i)) {
					pages.push(i);
				}
			}
			if (currentPage < totalPages - delta - 1) {
				pages.push('...');
			}
			if (!pages.includes(totalPages)) {
				pages.push(totalPages);
			}
		}

		return pages;
	}
</script>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Trading Room Archives</h2>
			
			{#if error}
				<div class="error-message">
					<p>{error}</p>
				</div>
			{/if}
			
			<!-- Filters -->
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <strong>{totalItems}</strong> sessions
				</div>
				<div class="dashboard-filters__search">
					<form onsubmit={handleSearchSubmit} class="search-form">
						<div class="search-input-wrapper">
							<svg class="search-icon" xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
								<circle cx="11" cy="11" r="8"></circle>
								<path d="m21 21-4.3-4.3"></path>
							</svg>
							<input 
								type="text" 
								placeholder="Search archives..." 
								bind:value={searchQuery}
								oninput={handleSearch}
								class="search-input"
							/>
							{#if searchQuery}
								<button type="button" class="clear-btn" onclick={clearSearch} aria-label="Clear search">
									<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">
										<path d="M18 6 6 18"></path>
										<path d="m6 6 12 12"></path>
									</svg>
								</button>
							{/if}
						</div>
					</form>
				</div>
			</div>

			<!-- Archive Grid -->
			<div id="products-list">
				{#each paginatedGroups as group (group.date)}
					<p class="date-heading"><strong>{group.displayDate}</strong></p>
					<div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-5">
						{#each group.sessions as session (session.id)}
							<article class="archive-card">
								<div class="card-body">
									<div class="card-content">
										<h4 class="card-title"><strong>{session.title}</strong></h4>
										{#if session.trader}
											<p class="card-trader"><i>With {session.trader}</i></p>
										{/if}
									</div>
									<a href={session.href} class="btn btn-tiny btn-default">
										Watch Now
									</a>
								</div>
							</article>
						{/each}
					</div>
				{/each}

				{#if paginatedGroups.length === 0 && !error}
					<div class="empty-state">
						<h3>No sessions found</h3>
						<p>Try adjusting your search query or check back later for new content.</p>
					</div>
				{/if}
			</div>

			<!-- Pagination -->
			{#if totalPages > 1}
				<div class="fl-builder-pagination">
					<ul class="page-numbers">
						{#each getPageNumbers() as page}
							{#if typeof page === 'number'}
								<li>
									<button 
										class="page-number"
										class:current={page === currentPage}
										onclick={() => goToPage(page)}
									>
										{page}
									</button>
								</li>
							{:else}
								<li>
									<span class="page-number dots">...</span>
								</li>
							{/if}
						{/each}
					</ul>
				</div>
			{/if}
		</section>
	</div>
</div>

<style>
	/* Dashboard Content */
	.dashboard__content {
		display: flex;
		flex-direction: column;
	}

	.dashboard__content-main {
		flex: 1;
		padding: 20px;
		background: #f5f5f5;
	}

	@media (min-width: 768px) {
		.dashboard__content-main {
			padding: 30px;
		}
	}

	.dashboard__content-section {
		background: #fff;
		border-radius: 5px;
		padding: 20px;
		box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
	}

	@media (min-width: 768px) {
		.dashboard__content-section {
			padding: 30px;
		}
	}

	/* Section Title */
	.section-title {
		font-size: 20px;
		font-weight: 700;
		color: #333;
		margin: 0 0 20px;
		font-family: 'Open Sans', sans-serif;
	}

	@media (min-width: 768px) {
		.section-title {
			font-size: 24px;
		}
	}

	/* Error Message */
	.error-message {
		background: #fee;
		border: 1px solid #fcc;
		color: #c00;
		padding: 15px 20px;
		border-radius: 4px;
		margin-bottom: 20px;
	}

	/* Dashboard Filters */
	.dashboard-filters {
		display: flex;
		flex-direction: column;
		gap: 15px;
		margin-bottom: 30px;
		padding-bottom: 20px;
		border-bottom: 1px solid #e6e6e6;
	}

	@media (min-width: 768px) {
		.dashboard-filters {
			flex-direction: row;
			justify-content: space-between;
			align-items: center;
		}
	}

	.dashboard-filters__count {
		font-size: 14px;
		color: #666;
	}

	.dashboard-filters__search {
		width: 100%;
	}

	@media (min-width: 768px) {
		.dashboard-filters__search {
			width: auto;
			min-width: 300px;
		}
	}

	.search-form {
		width: 100%;
	}

	.search-input-wrapper {
		position: relative;
		display: flex;
		align-items: center;
	}

	.search-icon {
		position: absolute;
		left: 12px;
		color: #999;
		pointer-events: none;
	}

	.search-input {
		width: 100%;
		padding: 10px 40px 10px 40px;
		border: 1px solid #e6e6e6;
		border-radius: 4px;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
	}

	.search-input:focus {
		outline: none;
		border-color: #0984ae;
		box-shadow: 0 0 0 3px rgba(9, 132, 174, 0.1);
	}

	.search-input::placeholder {
		color: #999;
	}

	.clear-btn {
		position: absolute;
		right: 10px;
		background: none;
		border: none;
		padding: 4px;
		cursor: pointer;
		color: #999;
		display: flex;
		align-items: center;
		justify-content: center;
		border-radius: 50%;
		transition: all 0.2s ease;
	}

	.clear-btn:hover {
		color: #666;
		background: #f0f0f0;
	}

	/* Date Heading */
	.date-heading {
		font-size: 16px;
		color: #333;
		margin: 0 0 15px;
		font-family: 'Open Sans', sans-serif;
	}

	/* Archive Card */
	.archive-card {
		background: #fff;
		border: 1px solid #e6e6e6;
		border-radius: 5px;
		overflow: hidden;
		transition: box-shadow 0.2s ease;
		height: 100%;
	}

	.archive-card:hover {
		box-shadow: 0 3px 10px rgba(0, 0, 0, 0.1);
	}

	.card-body {
		display: flex;
		flex-direction: column;
		justify-content: space-between;
		padding: 20px;
		min-height: 120px;
		height: 100%;
	}

	.card-content {
		margin-bottom: 15px;
	}

	.card-title {
		font-size: 16px;
		font-weight: 700;
		color: #333;
		margin: 0 0 5px;
		font-family: 'Open Sans', sans-serif;
	}

	.card-trader {
		font-size: 13px;
		color: #666;
		margin: 0;
	}

	/* Buttons */
	.btn {
		display: inline-block;
		padding: 10px 20px;
		font-size: 13px;
		font-weight: 600;
		text-align: center;
		text-decoration: none;
		border-radius: 4px;
		transition: all 0.2s ease;
		cursor: pointer;
		border: none;
	}

	.btn-tiny {
		padding: 8px 16px;
		font-size: 12px;
	}

	.btn-default {
		background: #143E59;
		color: #fff;
		border: 1px solid #143E59;
	}

	.btn-default:hover {
		background: #0c2638;
		border-color: #0c2638;
	}

	/* Empty State */
	.empty-state {
		text-align: center;
		padding: 60px 20px;
		color: #666;
	}

	.empty-state h3 {
		margin: 0 0 10px;
		font-size: 18px;
		color: #333;
	}

	.empty-state p {
		margin: 0;
		font-size: 14px;
	}

	/* Pagination */
	.fl-builder-pagination {
		padding: 40px 0 20px;
		text-align: center;
	}

	.page-numbers {
		list-style: none;
		margin: 0;
		padding: 0;
		display: flex;
		justify-content: center;
		flex-wrap: wrap;
		gap: 5px;
	}

	.page-number {
		display: inline-block;
		padding: 8px 14px;
		border: 1px solid #e6e6e6;
		background: #fff;
		color: #333;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		cursor: pointer;
		transition: all 0.2s ease;
	}

	.page-number:hover {
		background: #f5f5f5;
	}

	.page-number.current {
		background: #143E59;
		color: #fff;
		border-color: #143E59;
	}

	.page-number.dots {
		cursor: default;
		background: #fff;
	}

	.page-number.dots:hover {
		background: #fff;
	}

	/* Grid gap override */
	#products-list {
		margin-bottom: 20px;
	}

	#products-list > :global(.grid) {
		margin-bottom: 30px;
	}
</style>
