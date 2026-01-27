<!--
	Event Explorer Component
	═══════════════════════════════════════════════════════════════════════════
	
	Advanced event exploration with filtering, search, and breakdown analysis.
-->

<script lang="ts">
	import { onMount } from 'svelte';
	import { analyticsApi, type AnalyticsEvent } from '$lib/api/analytics';
	import { IconSearch, IconFilter, IconDownload, IconRefresh } from '$lib/icons';

	interface Props {
		period?: string;
	}

	let { period = '7d' }: Props = $props();

	let events: AnalyticsEvent[] = $state([]);
	let eventTypes: Array<{ name: string; count: number }> = $state([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let selectedEventType = $state('');
	let currentPage = $state(1);
	let totalPages = $state(1);

	async function loadEvents() {
		loading = true;
		try {
			const response = await analyticsApi.getEvents({
				period,
				event_type: selectedEventType || undefined,
				search: searchQuery || undefined,
				page: currentPage,
				per_page: 50
			});

			events = response.events;
			eventTypes = response.event_types;
			totalPages = response.pagination.total_pages;
		} catch (error) {
			console.error('Failed to load events:', error);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadEvents();
	});

	function handleSearch() {
		currentPage = 1;
		loadEvents();
	}

	function handleFilterChange() {
		currentPage = 1;
		loadEvents();
	}

	function formatDate(dateString: string): string {
		return new Date(dateString).toLocaleString();
	}
</script>

<div class="event-explorer">
	<div class="explorer-header">
		<h2 class="explorer-title">Event Explorer</h2>
		<button class="btn-refresh" onclick={loadEvents}>
			<IconRefresh size={18} />
			Refresh
		</button>
	</div>

	<!-- Filters -->
	<div class="explorer-filters">
		<div class="search-box">
			<IconSearch size={20} class="search-icon" />
			<input
				type="text"
				bind:value={searchQuery}
				onkeyup={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
				placeholder="Search events..."
				class="search-input"
			/>
		</div>

		<select bind:value={selectedEventType} onchange={handleFilterChange} class="filter-select">
			<option value="">All Event Types</option>
			{#each eventTypes as type}
				<option value={type.name}>{type.name} ({type.count})</option>
			{/each}
		</select>

		<button class="btn-export">
			<IconDownload size={18} />
			Export
		</button>
	</div>

	<!-- Events Table -->
	<div class="events-table-container">
		{#if loading}
			<div class="loading">
				<div class="spinner"></div>
				<p>Loading events...</p>
			</div>
		{:else if events.length > 0}
			<table class="events-table">
				<thead>
					<tr>
						<th>Event Name</th>
						<th>Type</th>
						<th>Category</th>
						<th>User ID</th>
						<th>Page</th>
						<th>Channel</th>
						<th>Timestamp</th>
					</tr>
				</thead>
				<tbody>
					{#each events as event}
						<tr>
							<td class="event-name">{event.event_name}</td>
							<td><span class="badge">{event.event_type}</span></td>
							<td>{event.event_category || '-'}</td>
							<td>{event.user_id || 'Anonymous'}</td>
							<td class="page-path">{event.page_path || '-'}</td>
							<td>{event.channel || '-'}</td>
							<td class="timestamp">{formatDate(event.created_at)}</td>
						</tr>
					{/each}
				</tbody>
			</table>

			<!-- Pagination -->
			<div class="pagination">
				<button
					class="page-btn"
					disabled={currentPage === 1}
					onclick={() => {
						currentPage--;
						loadEvents();
					}}
				>
					Previous
				</button>
				<span class="page-info">Page {currentPage} of {totalPages}</span>
				<button
					class="page-btn"
					disabled={currentPage === totalPages}
					onclick={() => {
						currentPage++;
						loadEvents();
					}}
				>
					Next
				</button>
			</div>
		{:else}
			<div class="empty-state">
				<IconFilter size={48} class="text-gray-600" />
				<p class="text-gray-400 mt-3">No events found</p>
			</div>
		{/if}
	</div>
</div>

<style lang="postcss">
	@reference "../../../app.css";
	.event-explorer {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.explorer-header {
		@apply flex items-center justify-between mb-6;
	}

	.explorer-title {
		@apply text-2xl font-bold text-white;
	}

	.btn-refresh {
		@apply flex items-center gap-2 px-4 py-2 bg-gray-700 text-white rounded-lg;
		@apply hover:bg-gray-600 transition-colors;
	}

	.explorer-filters {
		@apply flex gap-4 mb-6 flex-wrap;
	}

	.search-box {
		@apply relative flex-1 min-w-[300px];
	}

	.search-box :global(.search-icon) {
		@apply absolute left-3 top-1/2 -translate-y-1/2 text-gray-400;
	}

	.search-input {
		@apply w-full pl-10 pr-4 py-2 bg-gray-700 text-white rounded-lg border border-gray-600;
		@apply focus:outline-none focus:ring-2 focus:ring-yellow-500;
	}

	.filter-select {
		@apply px-4 py-2 bg-gray-700 text-white rounded-lg border border-gray-600;
		@apply focus:outline-none focus:ring-2 focus:ring-yellow-500;
	}

	.btn-export {
		@apply flex items-center gap-2 px-4 py-2 bg-yellow-500 text-gray-900 rounded-lg font-semibold;
		@apply hover:bg-yellow-400 transition-colors;
	}

	.events-table-container {
		@apply overflow-x-auto;
	}

	.events-table {
		@apply w-full text-sm;
	}

	.events-table thead {
		@apply bg-gray-900/50;
	}

	.events-table th {
		@apply px-4 py-3 text-left text-gray-400 font-semibold;
	}

	.events-table tbody tr {
		@apply border-b border-gray-700/50 hover:bg-gray-700/30 transition-colors;
	}

	.events-table td {
		@apply px-4 py-3 text-gray-300;
	}

	.event-name {
		@apply font-semibold text-white;
	}

	.badge {
		@apply px-2 py-1 bg-blue-500/20 text-blue-400 rounded text-xs font-medium;
	}

	.page-path {
		@apply text-xs text-gray-400 max-w-[200px] truncate;
	}

	.timestamp {
		@apply text-xs text-gray-500;
	}

	.pagination {
		@apply flex items-center justify-center gap-4 mt-6;
	}

	.page-btn {
		@apply px-4 py-2 bg-gray-700 text-white rounded-lg;
		@apply hover:bg-gray-600 transition-colors disabled:opacity-50 disabled:cursor-not-allowed;
	}

	.page-info {
		@apply text-gray-400;
	}

	.loading {
		@apply flex flex-col items-center justify-center py-12;
	}

	.spinner {
		@apply w-8 h-8 border-4 border-gray-700 border-t-yellow-400 rounded-full animate-spin;
	}

	.empty-state {
		@apply flex flex-col items-center justify-center py-12 text-center;
	}
</style>
