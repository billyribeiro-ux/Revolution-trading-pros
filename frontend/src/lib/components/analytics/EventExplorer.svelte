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
			{#each eventTypes as type (type.name)}
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
					{#each events as event (event.id ?? event.created_at)}
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
				<IconFilter size={48} class="empty-icon" />
				<p class="empty-copy">No events found</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.event-explorer {
		background-color: rgba(30, 41, 59, 0.5);
		border-radius: 0.75rem;
		padding: 1.5rem;
		border: 1px solid rgba(51, 65, 85, 0.5);
	}

	.explorer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.explorer-title {
		margin: 0;
		font-size: 1.5rem;
		line-height: 2rem;
		font-weight: 700;
		color: #ffffff;
	}

	.btn-refresh {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-refresh:hover {
		background: #4b5563;
	}

	.explorer-filters {
		display: flex;
		flex-wrap: wrap;
		gap: 1rem;
		margin-bottom: 1.5rem;
	}

	.search-box {
		position: relative;
		flex: 1;
		min-width: 300px;
	}

	.search-box :global(.search-icon) {
		position: absolute;
		top: 50%;
		left: 0.75rem;
		color: #9ca3af;
		transform: translateY(-50%);
	}

	.search-input {
		width: 100%;
		padding: 0.5rem 1rem 0.5rem 2.5rem;
		border: 1px solid #4b5563;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
	}

	.search-input:focus,
	.filter-select:focus {
		outline: 2px solid #eab308;
		outline-offset: 2px;
	}

	.filter-select {
		padding: 0.5rem 1rem;
		border: 1px solid #4b5563;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
	}

	.btn-export {
		display: inline-flex;
		align-items: center;
		gap: 0.5rem;
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #eab308;
		color: #111827;
		font: inherit;
		font-weight: 600;
		cursor: pointer;
		transition: background-color 0.2s ease;
	}

	.btn-export:hover {
		background: #facc15;
	}

	.events-table-container {
		overflow-x: auto;
	}

	.events-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.events-table thead {
		background: rgba(17, 24, 39, 0.5);
	}

	.events-table th {
		padding: 0.75rem 1rem;
		color: #9ca3af;
		font-weight: 600;
		text-align: left;
	}

	.events-table tbody tr {
		border-bottom: 1px solid rgba(55, 65, 81, 0.5);
		transition: background-color 0.2s ease;
	}

	.events-table tbody tr:hover {
		background: rgba(55, 65, 81, 0.3);
	}

	.events-table td {
		padding: 0.75rem 1rem;
		color: #d1d5db;
	}

	.event-name {
		color: #ffffff;
		font-weight: 600;
	}

	.badge {
		padding: 0.25rem 0.5rem;
		border-radius: 0.25rem;
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
		font-size: 0.75rem;
		font-weight: 500;
		white-space: nowrap;
	}

	.page-path {
		max-width: 200px;
		overflow: hidden;
		color: #9ca3af;
		font-size: 0.75rem;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.timestamp {
		color: #6b7280;
		font-size: 0.75rem;
		white-space: nowrap;
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 1rem;
		margin-top: 1.5rem;
	}

	.page-btn {
		padding: 0.5rem 1rem;
		border: 0;
		border-radius: 0.5rem;
		background: #374151;
		color: #ffffff;
		font: inherit;
		cursor: pointer;
		transition:
			background-color 0.2s ease,
			opacity 0.2s ease;
	}

	.page-btn:hover:not(:disabled) {
		background: #4b5563;
	}

	.page-btn:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	.page-info {
		color: #9ca3af;
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
		color: #9ca3af;
	}

	.spinner {
		width: 2rem;
		height: 2rem;
		border: 4px solid #374151;
		border-top-color: #facc15;
		border-radius: 999px;
		animation: event-spin 1s linear infinite;
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: 3rem;
		text-align: center;
	}

	.empty-state :global(.empty-icon) {
		color: #4b5563;
	}

	.empty-copy {
		margin: 0.75rem 0 0;
		color: #9ca3af;
	}

	@media (max-width: 640px) {
		.explorer-header {
			align-items: flex-start;
			flex-direction: column;
		}

		.search-box {
			min-width: 100%;
		}
	}

	@keyframes event-spin {
		to {
			transform: rotate(360deg);
		}
	}
</style>
