<!--
	Event Explorer Component
	═══════════════════════════════════════════════════════════════════════════
	
	Advanced event exploration with filtering, search, and breakdown analysis.
-->

<script lang="ts">
	import { logger } from '$lib/utils/logger';
	import { onMount } from 'svelte';
	import { analyticsApi, type AnalyticsEvent } from '$lib/api/analytics';
	import { Icon, IconDownload, IconFilter, IconRefresh, IconSearch } from '$lib/icons';

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
			logger.error('Failed to load events:', error);
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
			<Icon icon={IconRefresh} size={18} />
			Refresh
		</button>
	</div>

	<!-- Filters -->
	<div class="explorer-filters">
		<div class="search-box">
			<Icon icon={IconSearch} size={20} class="search-icon" />
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
			<Icon icon={IconDownload} size={18} />
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
				<Icon icon={IconFilter} size={48} />
				<p class="empty-text">No events found</p>
			</div>
		{/if}
	</div>
</div>

<style>
	.event-explorer {
		background-color: oklch(0.2 0.02 250 / 50%);
		border-radius: var(--radius-xl);
		padding: var(--space-6);
		border: 1px solid oklch(0.35 0.02 250 / 50%);
	}

	.explorer-header {
		display: flex;
		align-items: center;
		justify-content: space-between;
		margin-block-end: var(--space-6);
	}

	.explorer-title {
		font-size: var(--text-2xl);
		font-weight: var(--weight-bold);
		color: oklch(1 0 0);
	}

	.btn-refresh {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.45 0.01 250);
		}
	}

	.explorer-filters {
		display: flex;
		gap: var(--space-4);
		margin-block-end: var(--space-6);
		flex-wrap: wrap;
	}

	.search-box {
		position: relative;
		flex: 1;
		min-inline-size: 300px;
	}

	.search-box :global(.search-icon) {
		position: absolute;
		inset-inline-start: var(--space-3);
		inset-block-start: 50%;
		transform: translateY(-50%);
		color: oklch(0.65 0.01 250);
	}

	.search-input {
		inline-size: 100%;
		padding-inline-start: 2.5rem;
		padding-inline-end: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.45 0.01 250);
		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.8 0.18 90);
		}
	}

	.filter-select {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: 1px solid oklch(0.45 0.01 250);
		&:focus {
			outline: none;
			box-shadow: 0 0 0 2px oklch(0.8 0.18 90);
		}
	}

	.btn-export {
		display: flex;
		align-items: center;
		gap: var(--space-2);
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.8 0.18 90);
		color: oklch(0.15 0.02 90);
		border-radius: var(--radius-lg);
		font-weight: var(--weight-semibold);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.85 0.16 90);
		}
	}

	.events-table-container {
		overflow-x: auto;
	}

	.events-table {
		inline-size: 100%;
		font-size: var(--text-sm);

		& thead {
			background-color: oklch(0.15 0.01 250 / 50%);
		}

		& th {
			padding-inline: var(--space-4);
			padding-block: var(--space-3);
			text-align: start;
			color: oklch(0.65 0.01 250);
			font-weight: var(--weight-semibold);
		}

		& tbody tr {
			border-block-end: 1px solid oklch(0.38 0.01 250 / 50%);
			transition: background-color var(--duration-fast) var(--ease-default);
			&:hover {
				background-color: oklch(0.38 0.01 250 / 30%);
			}
		}

		& td {
			padding-inline: var(--space-4);
			padding-block: var(--space-3);
			color: oklch(0.75 0.01 250);
		}
	}

	.event-name {
		font-weight: var(--weight-semibold);
		color: oklch(1 0 0);
	}

	.badge {
		padding-inline: var(--space-2);
		padding-block: var(--space-1);
		background-color: oklch(0.6 0.2 260 / 20%);
		color: oklch(0.7 0.18 260);
		border-radius: var(--radius-sm);
		font-size: var(--text-xs);
		font-weight: var(--weight-medium);
	}

	.page-path {
		font-size: var(--text-xs);
		color: oklch(0.65 0.01 250);
		max-inline-size: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.timestamp {
		font-size: var(--text-xs);
		color: oklch(0.55 0.01 250);
	}

	.pagination {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: var(--space-4);
		margin-block-start: var(--space-6);
	}

	.page-btn {
		padding-inline: var(--space-4);
		padding-block: var(--space-2);
		background-color: oklch(0.38 0.01 250);
		color: oklch(1 0 0);
		border-radius: var(--radius-lg);
		border: none;
		cursor: pointer;
		transition: background-color var(--duration-fast) var(--ease-default);
		&:hover {
			background-color: oklch(0.45 0.01 250);
		}
		&:disabled {
			opacity: 0.5;
			cursor: not-allowed;
		}
	}

	.page-info {
		color: oklch(0.65 0.01 250);
	}

	.loading {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-12);
	}

	.spinner {
		inline-size: 2rem;
		block-size: 2rem;
		border: 4px solid oklch(0.38 0.01 250);
		border-block-start-color: oklch(0.8 0.18 90);
		border-radius: 9999px;
		animation: spin 1s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.empty-state {
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
		padding-block: var(--space-12);
		text-align: center;
		color: oklch(0.45 0.01 250);
	}

	.empty-text {
		color: oklch(0.65 0.01 250);
		margin-block-start: var(--space-3);
	}
</style>
