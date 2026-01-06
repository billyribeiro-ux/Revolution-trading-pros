<!--
	URL: /dashboard/day-trading-room/trading-room-archive
	
	Trading Room Archive Page
	═══════════════════════════════════════════════════════════════════════════
	Apple ICT 11+ Principal Engineer Implementation
	
	Archive of past live trading sessions and recordings.
	Includes search, date-grouped grid, and pagination.
	Connected to unified videos API with content_type=room_archive.
	
	@version 4.0.0
	@author Revolution Trading Pros
-->
<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import TradingRoomHeader from '$lib/components/dashboard/TradingRoomHeader.svelte';
	import type { PageData } from './$types';

	// Server data
	let { data }: { data: PageData } = $props();

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

	// Local search state (for client-side filtering)
	let searchQuery = $state(data.search || '');
	let currentPage = $derived(data.meta?.current_page || 1);
	let totalPages = $derived(data.meta?.last_page || 1);
	let totalItems = $derived(data.meta?.total || 0);

	// Transform API videos to sessions
	let allSessions = $derived<ArchiveSession[]>(
		(data.videos || []).map(video => ({
			id: String(video.id),
			title: video.title,
			trader: video.trader?.name || '',
			date: video.video_date?.split('T')[0] || '',
			href: `/chatroom-archive/day-trading-room/${video.slug}`
		}))
	);

	// Client-side filtering (when user types in search)
	let filteredSessions = $derived(
		searchQuery.trim() === '' || searchQuery === data.search
			? allSessions
			: allSessions.filter(session => 
				session.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
				session.trader.toLowerCase().includes(searchQuery.toLowerCase())
			)
	);

	// Format date for display
	function formatDate(dateStr: string): string {
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
	function handleSearch(event: Event) {
		const target = event.target as HTMLInputElement;
		searchQuery = target.value;
		
		// Debounce search to avoid too many navigations
		clearTimeout(searchTimeout);
		searchTimeout = setTimeout(() => {
			const params = new URLSearchParams();
			if (searchQuery.trim()) {
				params.set('search', searchQuery.trim());
			}
			params.set('page', '1');
			goto(`?${params.toString()}`, { replaceState: true, noScroll: true });
		}, 300);
	}

	// Handle page change - navigate to update URL params
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

<svelte:head>
	<title>Trading Room Archives | Day Trading Room | Revolution Trading Pros</title>
	<meta name="description" content="Access recordings of past live trading sessions and chat logs." />
	<meta name="robots" content="noindex, nofollow" />
</svelte:head>

<TradingRoomHeader 
	roomName="Day Trading Room" 
	pageTitle="Trading Room Archives"
	startHereUrl="/dashboard/day-trading-room/start-here" 
/>

<div class="dashboard__content">
	<div class="dashboard__content-main">
		<section class="dashboard__content-section">
			<h2 class="section-title">Trading Room Archives</h2>
			
			<!-- Filters -->
			<div class="dashboard-filters">
				<div class="dashboard-filters__count">
					Showing <strong>{totalItems}</strong> sessions
				</div>
				<div class="dashboard-filters__search">
					<input 
						type="text" 
						placeholder="Search archives..." 
						value={searchQuery}
						oninput={handleSearch}
						class="search-input"
					/>
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

				{#if paginatedGroups.length === 0}
					<div class="empty-state">
						<h3>No sessions found</h3>
						<p>Try adjusting your search query.</p>
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

	.search-input {
		width: 100%;
		padding: 10px 15px;
		border: 1px solid #e6e6e6;
		border-radius: 4px;
		font-size: 14px;
		font-family: 'Open Sans', sans-serif;
		transition: border-color 0.2s ease;
	}

	.search-input:focus {
		outline: none;
		border-color: #0984ae;
	}

	.search-input::placeholder {
		color: #999;
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

	/* Mobile-first grid gap override */
	#products-list {
		margin-bottom: 20px;
	}

	#products-list > .grid {
		margin-bottom: 30px;
	}
</style>
