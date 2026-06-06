<script lang="ts">
	/**
	 * Event Explorer - Advanced Event Analysis Tool
	 * Apple ICT7 Grade Implementation
	 *
	 * Browse, search, and analyze tracked events with
	 * filtering, grouping, and visualization capabilities.
	 */
	import { onMount } from 'svelte';
	import { browser } from '$app/environment';
	import { analyticsApi, type AnalyticsEvent } from '$lib/api/analytics';
	import { connections, getIsAnalyticsConnected } from '$lib/stores/connections.svelte';
	import ServiceConnectionStatus from '$lib/components/admin/ServiceConnectionStatus.svelte';
	import PeriodSelector from '$lib/components/analytics/PeriodSelector.svelte';
	// FIX-2026-04-26: Tabler icons replace raw inline <svg> blocks.
	import IconBolt from '@tabler/icons-svelte-runes/icons/bolt';
	import IconSearch from '@tabler/icons-svelte-runes/icons/search';
	import IconAlertCircle from '@tabler/icons-svelte-runes/icons/alert-circle';

	// Svelte 5 Runes - State
	let events = $state<AnalyticsEvent[]>([]);
	let eventTypes = $state<{ name: string; count: number }[]>([]);
	let loading = $state(true);
	let connectionLoading = $state(true);
	let error = $state<string | null>(null);
	let selectedPeriod = $state('7d');
	let selectedEventType = $state('');
	let searchQuery = $state('');
	let page = $state(1);
	let totalPages = $state(1);
	let totalEvents = $state(0);

	// Event type tones - Apple-inspired palette.
	const eventToneClasses: Record<string, string> = {
		page_view: 'event-chip--blue',
		click: 'event-chip--emerald',
		form_submit: 'event-chip--purple',
		purchase: 'event-chip--amber',
		signup: 'event-chip--pink',
		login: 'event-chip--cyan',
		error: 'event-chip--red',
		custom: 'event-chip--slate'
	};

	async function loadEvents() {
		loading = true;
		error = null;
		try {
			const response = await analyticsApi.getEvents({
				period: selectedPeriod,
				...(selectedEventType && { event_type: selectedEventType }),
				...(searchQuery && { search: searchQuery }),
				page
			});
			events = response.events;
			eventTypes = response.event_types || [];
			totalPages = response.pagination?.total_pages || 1;
			totalEvents = response.pagination?.total || 0;
		} catch (e) {
			error = e instanceof Error ? e.message : 'Failed to load events';
		} finally {
			loading = false;
		}
	}

	function handlePeriodChange(value: string) {
		selectedPeriod = value;
		page = 1;
		loadEvents();
	}

	function handleEventTypeChange() {
		page = 1;
		loadEvents();
	}

	function handleSearch() {
		page = 1;
		loadEvents();
	}

	function formatDate(dateStr: string): string {
		const date = new Date(dateStr);
		return date.toLocaleString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit'
		});
	}

	function formatProperties(props: Record<string, unknown>): string {
		// FIX-2026-04-26 (P3-5): stringify once, not twice.
		if (!props || Object.keys(props).length === 0) return '-';
		const json = JSON.stringify(props);
		return json.slice(0, 100) + (json.length > 100 ? '...' : '');
	}

	function getEventToneClass(eventType: string) {
		return eventToneClasses[eventType] ?? 'event-chip--slate';
	}

	// FIX-2026-04-26 (P1-3): $derived restores reactivity past helper's `untrack`.
	let isAnalyticsConnected = $derived(getIsAnalyticsConnected());

	// FIX-2026-04-26 (P1-1): onMount replaces the $effect cascade pattern.
	onMount(() => {
		if (!browser) return;

		(async () => {
			try {
				await connections.load();
			} catch (e) {
				if (import.meta.env.DEV) {
					console.error('[Events] Failed to load connection status:', e);
				}
			} finally {
				connectionLoading = false;
			}

			if (getIsAnalyticsConnected()) {
				await loadEvents();
			} else {
				loading = false;
			}
		})();
	});
</script>

<svelte:head>
	<title>Event Explorer | Analytics</title>
</svelte:head>

<div class="events-page">
	<div class="events-container">
		<!-- Apple ICT7 Grade Header -->
		<header class="events-header">
			<div class="header-title-group">
				<div class="header-icon">
					<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (events) -->
					<IconBolt size={24} aria-hidden="true" />
				</div>
				<div>
					<h1 class="page-title">Event Explorer</h1>
					<p class="page-subtitle">Browse and analyze tracked events in real-time</p>
				</div>
			</div>
		</header>

		<!-- Connection Check -->
		{#if connectionLoading}
			<div class="loading-state">
				<div class="spinner spinner--large"></div>
			</div>
		{:else if !isAnalyticsConnected}
			<ServiceConnectionStatus feature="analytics" variant="card" showFeatures={true} />
		{:else}
			<!-- Filters - Glass morphism card -->
			<div class="filters-panel">
				<div class="filters-row">
					<PeriodSelector value={selectedPeriod} onchange={handlePeriodChange} />

					<select
						bind:value={selectedEventType}
						onchange={handleEventTypeChange}
						class="event-select"
						aria-label="Event type"
					>
						<option value="">All Event Types</option>
						{#each eventTypes as type (type.name)}
							<option value={type.name}>{type.name} ({type.count})</option>
						{/each}
					</select>

					<div class="search-column">
						<div class="search-field">
							<input
								id="page-searchquery"
								name="page-searchquery"
								type="text"
								bind:value={searchQuery}
								onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && handleSearch()}
								placeholder="Search events..."
								class="search-input"
							/>
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: search (search input) -->
							<span class="search-icon">
								<IconSearch size={16} aria-hidden="true" />
							</span>
						</div>
					</div>

					<button onclick={handleSearch} class="search-button"> Search </button>
				</div>
			</div>

			<!-- Event Type Distribution -->
			{#if eventTypes.length > 0}
				<div class="distribution-panel">
					<h3 class="panel-title">Event Distribution</h3>
					<div class="event-chip-list">
						{#each eventTypes.slice(0, 12) as type (type.name)}
							<button
								onclick={() => {
									selectedEventType = type.name;
									handleEventTypeChange();
								}}
								class={[
									'event-filter-chip',
									selectedEventType === type.name
										? 'event-filter-chip--active'
										: getEventToneClass(type.name)
								]}
							>
								{type.name}
								<span class="event-chip-count">{type.count.toLocaleString()}</span>
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Events Table -->
			<div class="events-panel">
				<div class="events-panel-header">
					<h3 class="events-title">
						Events
						<span class="events-count">
							({totalEvents.toLocaleString()} total)
						</span>
					</h3>
					<button class="export-button"> Export CSV </button>
				</div>

				{#if loading}
					<div class="loading-state">
						<div class="spinner"></div>
					</div>
				{:else if error}
					<div class="state-panel">
						<div class="state-icon state-icon--error">
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: alert-circle (error state) -->
							<IconAlertCircle size={32} aria-hidden="true" />
						</div>
						<p class="error-message">{error}</p>
						<button onclick={loadEvents} class="retry-button"> Retry </button>
					</div>
				{:else if events.length === 0}
					<div class="state-panel state-panel--empty">
						<div class="state-icon state-icon--amber">
							<!-- FIX-2026-04-26: replaced raw SVG with Tabler icon. Old: bolt (no events empty state) -->
							<IconBolt size={32} aria-hidden="true" />
						</div>
						<h3 class="state-title">No Events Found</h3>
						<p class="state-copy">Try adjusting your filters or time range</p>
					</div>
				{:else}
					<div class="table-scroll">
						<table class="events-table">
							<thead>
								<tr>
									<th>Event</th>
									<th>User</th>
									<th>Page</th>
									<th>Properties</th>
									<th>Source</th>
									<th class="align-right">Time</th>
								</tr>
							</thead>
							<tbody>
								{#each events as event (event.id)}
									<tr class="event-row">
										<td>
											<span class={['event-badge', getEventToneClass(event.event_type)]}>
												{event.event_name}
											</span>
										</td>
										<td>
											{#if event.user_id}
												<span class="user-id">{event.user_id}</span>
											{:else}
												<span class="anonymous-user">Anonymous</span>
											{/if}
										</td>
										<td class="path-cell">
											{event.page_path || '-'}
										</td>
										<td class="properties-cell">
											{formatProperties(event.properties || {})}
										</td>
										<td>
											<span class="source-cell">
												{event.channel || '-'}
											</span>
										</td>
										<td class="time-cell">
											{formatDate(event.created_at)}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>

					<!-- Pagination -->
					{#if totalPages > 1}
						<div class="pagination">
							<p class="pagination-label">
								Page {page} of {totalPages}
							</p>
							<div class="pagination-actions">
								<button
									onclick={() => {
										page = Math.max(1, page - 1);
										loadEvents();
									}}
									disabled={page === 1}
									class="pagination-button"
								>
									Previous
								</button>
								<button
									onclick={() => {
										page = Math.min(totalPages, page + 1);
										loadEvents();
									}}
									disabled={page === totalPages}
									class="pagination-button"
								>
									Next
								</button>
							</div>
						</div>
					{/if}
				{/if}
			</div>
		{/if}
	</div>
</div>

<style>
	.events-page {
		min-height: 100%;
		background: linear-gradient(135deg, #020617 0%, #0f172a 50%, #020617 100%);
		color: #f8fafc;
	}

	.events-container {
		width: min(100%, 80rem);
		margin: 0 auto;
		padding: 2rem 1rem;
	}

	.header-title-group,
	.header-icon,
	.loading-state,
	.filters-row,
	.search-field,
	.event-chip-list,
	.events-panel-header,
	.state-icon,
	.pagination,
	.pagination-actions {
		display: flex;
	}

	.events-header {
		margin-bottom: 2rem;
	}

	.header-title-group {
		align-items: center;
		gap: 1rem;
		margin-bottom: 0.5rem;
	}

	.header-icon {
		width: 3rem;
		height: 3rem;
		align-items: center;
		justify-content: center;
		border-radius: 1rem;
		background: linear-gradient(135deg, #f59e0b, #ea580c);
		box-shadow: 0 12px 26px rgba(245, 158, 11, 0.2);
		color: #ffffff;
	}

	.page-title {
		margin: 0;
		color: #ffffff;
		font-size: 1.5rem;
		font-weight: 700;
		letter-spacing: 0;
		line-height: 1.2;
	}

	.page-subtitle {
		margin: 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.loading-state {
		align-items: center;
		justify-content: center;
		padding: 5rem 0;
	}

	.spinner {
		width: 2.5rem;
		height: 2.5rem;
		border: 4px solid rgba(245, 158, 11, 0.2);
		border-top-color: transparent;
		border-right-color: #f59e0b;
		border-radius: 999px;
		animation: spin 0.8s linear infinite;
	}

	.spinner--large {
		width: 3rem;
		height: 3rem;
	}

	.filters-panel,
	.distribution-panel,
	.events-panel {
		border: 1px solid rgba(255, 255, 255, 0.1);
		background: rgba(255, 255, 255, 0.05);
		backdrop-filter: blur(24px);
		-webkit-backdrop-filter: blur(24px);
	}

	.filters-panel,
	.distribution-panel {
		margin-bottom: 1.5rem;
		border-radius: 1rem;
	}

	.filters-panel {
		padding: 1.25rem;
	}

	.filters-row {
		flex-wrap: wrap;
		align-items: center;
		gap: 1rem;
	}

	.event-select,
	.search-input {
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		background: rgba(30, 41, 59, 0.5);
		color: #ffffff;
		font: inherit;
		font-size: 0.875rem;
		outline: none;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.event-select {
		padding: 0.625rem 1rem;
	}

	.event-select:focus,
	.search-input:focus {
		border-color: rgba(245, 158, 11, 0.5);
		box-shadow: 0 0 0 2px rgba(245, 158, 11, 0.25);
	}

	.search-column {
		min-width: min(100%, 12.5rem);
		flex: 1;
	}

	.search-field {
		position: relative;
	}

	.search-input {
		width: 100%;
		padding: 0.625rem 1rem 0.625rem 2.75rem;
	}

	.search-input::placeholder {
		color: #64748b;
	}

	.search-icon {
		position: absolute;
		top: 50%;
		left: 1rem;
		display: inline-flex;
		color: #64748b;
		transform: translateY(-50%);
	}

	.search-button,
	.event-filter-chip--active {
		border-color: transparent;
		background: linear-gradient(90deg, #f59e0b, #ea580c);
		box-shadow: 0 12px 26px rgba(245, 158, 11, 0.25);
		color: #ffffff;
	}

	.search-button {
		border: 0;
		border-radius: 0.75rem;
		padding: 0.625rem 1.25rem;
		font-size: 0.875rem;
		font-weight: 600;
		transition:
			background 0.2s ease,
			box-shadow 0.2s ease;
	}

	.search-button:hover,
	.search-button:focus-visible {
		background: linear-gradient(90deg, #fbbf24, #f97316);
		box-shadow: 0 14px 30px rgba(245, 158, 11, 0.4);
	}

	.distribution-panel {
		padding: 1.5rem;
	}

	.panel-title {
		margin: 0 0 1rem;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 600;
	}

	.event-chip-list {
		flex-wrap: wrap;
		gap: 0.5rem;
	}

	.event-filter-chip,
	.event-badge {
		border: 1px solid;
		font-weight: 500;
		transition:
			background 0.2s ease,
			border-color 0.2s ease,
			box-shadow 0.2s ease,
			color 0.2s ease;
	}

	.event-filter-chip {
		border-radius: 0.75rem;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
	}

	.event-filter-chip:not(.event-filter-chip--active):hover,
	.event-filter-chip:not(.event-filter-chip--active):focus-visible {
		background: rgba(51, 65, 85, 0.9);
	}

	.event-chip-count {
		margin-left: 0.5rem;
		opacity: 0.75;
	}

	.event-chip--blue {
		border-color: rgba(59, 130, 246, 0.3);
		background: rgba(59, 130, 246, 0.2);
		color: #60a5fa;
	}

	.event-chip--emerald {
		border-color: rgba(16, 185, 129, 0.3);
		background: rgba(16, 185, 129, 0.2);
		color: #34d399;
	}

	.event-chip--purple {
		border-color: rgba(168, 85, 247, 0.3);
		background: rgba(168, 85, 247, 0.2);
		color: #c084fc;
	}

	.event-chip--amber {
		border-color: rgba(245, 158, 11, 0.3);
		background: rgba(245, 158, 11, 0.2);
		color: #fbbf24;
	}

	.event-chip--pink {
		border-color: rgba(236, 72, 153, 0.3);
		background: rgba(236, 72, 153, 0.2);
		color: #f472b6;
	}

	.event-chip--cyan {
		border-color: rgba(6, 182, 212, 0.3);
		background: rgba(6, 182, 212, 0.2);
		color: #22d3ee;
	}

	.event-chip--red {
		border-color: rgba(239, 68, 68, 0.3);
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
	}

	.event-chip--slate {
		border-color: rgba(71, 85, 105, 0.5);
		background: rgba(51, 65, 85, 0.5);
		color: #cbd5e1;
	}

	.events-panel {
		overflow: hidden;
		border-radius: 1rem;
	}

	.events-panel-header {
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-bottom: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.25rem;
	}

	.events-title {
		margin: 0;
		color: #ffffff;
		font-weight: 600;
	}

	.events-count {
		margin-left: 0.5rem;
		color: #94a3b8;
		font-size: 0.875rem;
		font-weight: 400;
	}

	.export-button,
	.pagination-button {
		border: 1px solid rgba(255, 255, 255, 0.1);
		border-radius: 0.75rem;
		background: transparent;
		color: #cbd5e1;
		padding: 0.5rem 1rem;
		font-size: 0.875rem;
		transition:
			background 0.2s ease,
			color 0.2s ease,
			opacity 0.2s ease;
	}

	.export-button:hover,
	.export-button:focus-visible,
	.pagination-button:hover,
	.pagination-button:focus-visible {
		background: rgba(255, 255, 255, 0.05);
		color: #ffffff;
	}

	.state-panel {
		padding: 2rem;
		text-align: center;
	}

	.state-panel--empty {
		padding: 3rem 1.5rem;
	}

	.state-icon {
		width: 4rem;
		height: 4rem;
		align-items: center;
		justify-content: center;
		margin: 0 auto 1rem;
		border-radius: 1rem;
	}

	.state-icon--error {
		background: rgba(239, 68, 68, 0.1);
		color: #f87171;
	}

	.state-icon--amber {
		background: rgba(245, 158, 11, 0.1);
		color: #fbbf24;
	}

	.error-message {
		margin: 0 0 1rem;
		color: #f87171;
	}

	.retry-button {
		border: 1px solid rgba(239, 68, 68, 0.3);
		border-radius: 0.75rem;
		background: rgba(239, 68, 68, 0.2);
		color: #f87171;
		padding: 0.625rem 1.25rem;
		transition: background 0.2s ease;
	}

	.retry-button:hover,
	.retry-button:focus-visible {
		background: rgba(239, 68, 68, 0.3);
	}

	.state-title {
		margin: 0 0 0.5rem;
		color: #ffffff;
		font-size: 1.125rem;
		font-weight: 500;
	}

	.state-copy {
		margin: 0;
		color: #94a3b8;
	}

	.table-scroll {
		overflow-x: auto;
	}

	.events-table {
		width: 100%;
		min-width: 60rem;
		border-collapse: collapse;
		font-size: 0.875rem;
	}

	.events-table thead {
		background: rgba(30, 41, 59, 0.5);
	}

	.events-table th,
	.events-table td {
		padding: 1rem 1.25rem;
	}

	.events-table th {
		color: #94a3b8;
		font-size: 0.75rem;
		font-weight: 500;
		letter-spacing: 0.04em;
		text-align: left;
		text-transform: uppercase;
	}

	.events-table tbody tr + tr {
		border-top: 1px solid rgba(255, 255, 255, 0.05);
	}

	.event-row {
		cursor: pointer;
		transition: background 0.2s ease;
	}

	.event-row:hover {
		background: rgba(255, 255, 255, 0.05);
	}

	.event-badge {
		display: inline-flex;
		border-radius: 0.5rem;
		padding: 0.375rem 0.75rem;
		font-size: 0.75rem;
	}

	.user-id {
		color: #ffffff;
	}

	.anonymous-user {
		color: #64748b;
		font-style: italic;
	}

	.path-cell,
	.properties-cell {
		max-width: 12.5rem;
		overflow: hidden;
		text-overflow: ellipsis;
		white-space: nowrap;
	}

	.path-cell,
	.time-cell {
		color: #94a3b8;
	}

	.properties-cell {
		color: #64748b;
		font-family:
			ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New',
			monospace;
		font-size: 0.75rem;
	}

	.source-cell {
		color: #64748b;
		font-size: 0.75rem;
		text-transform: capitalize;
	}

	.time-cell,
	.align-right {
		text-align: right;
	}

	.time-cell {
		white-space: nowrap;
	}

	.pagination {
		align-items: center;
		justify-content: space-between;
		gap: 1rem;
		border-top: 1px solid rgba(255, 255, 255, 0.1);
		padding: 1.25rem;
	}

	.pagination-label {
		margin: 0;
		color: #94a3b8;
		font-size: 0.875rem;
	}

	.pagination-actions {
		align-items: center;
		gap: 0.5rem;
	}

	.pagination-button:disabled {
		cursor: not-allowed;
		opacity: 0.5;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	@media (min-width: 640px) {
		.events-container {
			padding-right: 1.5rem;
			padding-left: 1.5rem;
		}
	}

	@media (min-width: 1024px) {
		.events-container {
			padding-right: 2rem;
			padding-left: 2rem;
		}
	}

	@media (max-width: 767px) {
		.events-panel-header,
		.pagination {
			align-items: flex-start;
			flex-direction: column;
		}

		.event-select,
		.search-column,
		.search-button,
		.export-button,
		.pagination-actions,
		.pagination-button {
			width: 100%;
		}
	}
</style>
